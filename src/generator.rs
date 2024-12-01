use crate::datalayer::Datalayer;
use crate::world::DataWorld;
use bevy_app::{App, Plugins};
use bevy_ecs::bundle::Bundle;
use bevy_ecs::prelude;
use bevy_ecs::prelude::{EntityWorldMut, IntoSystem, SystemInput, World};
use leptos::prelude::*;
use leptos_axum::generate_route_list_with_exclusions_and_ssg_and_context;
use std::sync::{Arc, Mutex};

#[cfg(feature = "development")]
use leptos_axum::LeptosRoutes;

/// The static site generator.
///
/// You can manipulate the datalayer through adding [`Plugins`] and [`Resource`]s to the generator.
pub struct Generator {
    /// The Bevy [`App`] that holds the ECS world used as datalayer
    pub app: App,
}

impl Generator {
    /// Create a new, empty [`Generator`]
    pub fn new() -> Self {
        Generator { app: App::new() }
    }

    /// Insert a [`Resource`] to the [`Generator`]
    pub fn insert_resource<R: prelude::Resource>(&mut self, value: R) -> &mut Self {
        self.app.insert_resource(value);
        self
    }

    /// Add ECS [`Plugins`] to the [`Generator`]
    pub fn add_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self {
        self.app.add_plugins(plugins);
        self
    }

    /// Build the static site.
    ///
    /// If the `development` feature is enabled, this method will serve the static site using
    /// axum server.
    pub async fn build<IV>(&mut self, shell_fn: fn(LeptosOptions) -> IV) -> std::io::Result<()>
    where
        IV: IntoView + 'static,
    {
        self.app.update();
        let world = std::mem::replace(self.app.world_mut(), World::new());
        let datalayer = Datalayer::new(world);
        let data = Arc::new(Mutex::new(datalayer));
        let data_for_route_generation = data.clone();

        let conf = get_configuration(None).unwrap();
        let leptos_options = conf.leptos_options.clone();

        #[allow(unused)]
        let (routes, static_data_map) = generate_route_list_with_exclusions_and_ssg_and_context(
            move || shell_fn(leptos_options.clone()),
            None,
            move || provide_context(data_for_route_generation.clone()),
        );

        static_data_map.generate(&conf.leptos_options).await;

        #[cfg(feature = "development")]
        {
            use axum::Router;

            let addr = conf.leptos_options.site_addr;
            println!("listening on http://{}", &addr);

            let app = Router::new()
                .leptos_routes_with_context(
                    &conf.leptos_options,
                    routes,
                    move || provide_context(data.clone()),
                    {
                        let leptos_options = conf.leptos_options.clone();
                        move || shell_fn(leptos_options.clone())
                    },
                )
                .fallback(leptos_axum::file_and_error_handler(shell_fn))
                .with_state(conf.leptos_options);
            let listener = tokio::net::TcpListener::bind(&addr).await?;
            axum::serve(listener, app.into_make_service())
                .await
                .expect("Failed to start development server");
        }
        Ok(())
    }
}

impl DataWorld for Generator {
    fn run<S, In, Out, Marker>(&mut self, system: S, input: In::Inner<'_>) -> Out
    where
        S: IntoSystem<In, Out, Marker> + 'static,
        Out: 'static,
        In: SystemInput + 'static,
    {
        self.app
            .world_mut()
            .run_system_cached_with(system, input)
            .expect("Failed to execute system")
    }

    fn get_resource<R: prelude::Resource + Clone>(&self) -> Option<R> {
        self.app.world().get_resource::<R>().cloned()
    }

    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut {
        self.app.world_mut().spawn(bundle)
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}
