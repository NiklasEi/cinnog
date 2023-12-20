use bevy_ecs::bundle::Bundle;
use bevy_ecs::system::{BoxedSystem, IntoSystem, Resource};
use bevy_ecs::world::{EntityWorldMut, World};
use leptos::{expect_context, get_configuration, provide_context, IntoView, LeptosOptions};
use leptos_actix::generate_route_list_with_exclusions_and_ssg_and_context;
use leptos_router::{RouteListing, StaticDataMap, StaticParamsMap, StaticPath};
use std::any::type_name;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct DataLayer {
    world: World,
}

impl DataLayer {
    pub fn new() -> Self {
        DataLayer {
            world: World::new(),
        }
    }

    pub async fn build<IV>(
        self,
        app_fn: impl Fn() -> IV + Clone + Send + 'static,
    ) -> std::io::Result<()>
    where
        IV: IntoView + 'static,
    {
        let data = Arc::new(Mutex::new(self));
        let data_for_route_generation = data.clone();

        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;

        let (routes, static_data_map) = generate_route_list_with_exclusions_and_ssg_and_context(
            app_fn.clone(),
            None,
            move || provide_context(data_for_route_generation.clone()),
        );
        build_static_routes_with_world(
            &leptos_options,
            app_fn.clone(),
            data.clone(),
            &routes,
            &static_data_map,
        )
        .await
        .expect("Failed to build static routes");

        #[cfg(feature = "development")]
        {
            use actix_files::Files;
            use actix_web::web;
            use leptos_actix::LeptosRoutes;

            let addr = leptos_options.site_addr;
            println!("listening on http://{}", &addr);

            actix_web::HttpServer::new(move || {
                let site_root = &leptos_options.site_root;

                actix_web::App::new()
                    .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
                    // serve JS/WASM/CSS from `pkg`
                    .service(Files::new("/pkg", format!("{site_root}/pkg")))
                    // serve other assets from the `assets` directory
                    .service(Files::new("/assets", site_root))
                    .leptos_routes(leptos_options.to_owned(), routes.to_owned(), app_fn.clone())
                    .app_data(web::Data::new(leptos_options.to_owned()))
            })
            .bind(&addr)?
            .run()
            .await
        }
        #[cfg(not(feature = "development"))]
        Ok(())
    }

    pub fn insert_resource<R: Resource>(&mut self, value: R) {
        self.world.insert_resource(value)
    }

    pub fn get_resource<R: Resource + Clone>(&self) -> Option<R> {
        self.world.get_resource::<R>().cloned()
    }

    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut {
        self.world.spawn(bundle)
    }

    pub fn run<T: 'static>(&mut self, system: &mut BoxedSystem<(), T>) -> T {
        system.initialize(&mut self.world);
        system.run((), &mut self.world)
    }
}

impl Default for DataLayer {
    fn default() -> Self {
        Self::new()
    }
}

pub fn expect_resource<R: Resource + Clone>() -> R {
    use_resource::<R>().unwrap_or_else(|| {
        panic!(
            "Expected resource {}, but it didn't exist",
            type_name::<R>()
        )
    })
}

pub fn run_system<S, R, T>(system: S) -> R
where
    S: IntoSystem<(), R, T>,
    R: 'static,
{
    let data_layer = expect_context::<Arc<Mutex<DataLayer>>>();
    let mut lock = data_layer.lock().unwrap();
    let mut boxed_system: BoxedSystem<(), R> = Box::new(IntoSystem::into_system(system));

    lock.run(&mut boxed_system)
}

pub fn use_resource<R: Resource + Clone>() -> Option<R> {
    let data_layer = expect_context::<Arc<Mutex<DataLayer>>>();
    let lock = data_layer.lock().unwrap();
    lock.get_resource()
}

async fn build_static_routes_with_world<IV>(
    options: &LeptosOptions,
    app_fn: impl Fn() -> IV + 'static + Clone,
    world: Arc<Mutex<DataLayer>>,
    routes: &[RouteListing],
    static_data_map: &StaticDataMap,
) -> Result<(), std::io::Error>
where
    IV: IntoView + 'static,
{
    let mut static_data: HashMap<&str, StaticParamsMap> = HashMap::new();
    for (key, value) in static_data_map {
        match value {
            Some(value) => {
                let boxed_system_mutex = value.clone();
                let mut boxed_system = boxed_system_mutex.lock().unwrap();
                let mut world = world.lock().unwrap();
                let params = world.run(&mut boxed_system);
                static_data.insert(key, params);
            }
            None => {
                static_data.insert(key, StaticParamsMap::default());
            }
        };
    }
    let static_routes = routes
        .iter()
        .filter(|route| route.static_mode().is_some())
        .collect::<Vec<_>>();
    for route in static_routes {
        let mut path = StaticPath::new(route.leptos_path());
        for p in path.parents().into_iter().rev() {
            if let Some(data) = static_data.get(p.path()) {
                path.add_params(data);
            }
        }
        if let Some(data) = static_data.get(path.path()) {
            path.add_params(data);
        }
        for path in path.into_paths() {
            println!("building static route: {}", path);
            let world = world.clone();
            path.write(options, app_fn.clone(), move || {
                provide_context(world.clone())
            })
            .await?;
        }
    }
    Ok(())
}
