use crate::datalayer::Datalayer;
use crate::world::DataWorld;
use axum::response::Response as AxumResponse;
use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::IntoResponse,
};
use bevy_app::{App, Plugins};
use bevy_ecs::bundle::Bundle;
use bevy_ecs::prelude;
use bevy_ecs::prelude::{EntityWorldMut, IntoSystem, SystemInput, World};
use leptos::prelude::*;
use leptos_axum::{generate_route_list_with_exclusions_and_ssg_and_context, LeptosRoutes};
use std::sync::{Arc, Mutex};
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub struct Generator {
    pub app: App,
}

impl Generator {
    pub fn new() -> Self {
        Generator { app: App::new() }
    }

    pub fn insert_resource<R: prelude::Resource>(&mut self, value: R) -> &mut Self {
        self.app.insert_resource(value);
        self
    }

    pub fn add_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self {
        self.app.add_plugins(plugins);
        self
    }

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

pub async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    _req: Request<Body>,
) -> AxumResponse {
    let root = options.site_root.clone();
    let res = get_static_file(uri.clone(), &root).await.unwrap();
    if res.status() == StatusCode::NOT_FOUND {
        // try with `.html`
        let uri_html = format!("{}.html", uri).parse().unwrap();
        get_static_file(uri_html, &root)
            .await
            .unwrap()
            .into_response()
    } else {
        res.into_response()
    }
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<Body>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.into_response()),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}
