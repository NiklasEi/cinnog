use bevy_core::Name;
use bevy_ecs::schedule::{ExecutorKind, Schedule};
use bevy_ecs::world::World;
use ecs_leptos_ssg::app::{App, Person, TestResource};
use leptos::{provide_context, IntoView, LeptosOptions};
use leptos_actix::generate_route_list_with_exclusions_and_ssg;
use leptos_router::{RouteListing, StaticDataMap, StaticParamsMap, StaticPath};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use leptos::*;

    let conf = get_configuration(Some("./Cargo.toml")).await.unwrap();
    let leptos_options = conf.leptos_options;

    let mut world = World::new();
    let mut schedule = Schedule::default();
    schedule.set_executor_kind(ExecutorKind::SingleThreaded);
    world.insert_resource(TestResource("testing 123".to_owned()));
    world.spawn((Person, Name::new("valentin")));
    world.spawn((Person, Name::new("stephan")));
    schedule.run(&mut world);
    let world = Arc::new(Mutex::new(world));
    let world_for_route_generation = world.clone();

    let (routes, static_data_map) =
        generate_route_list_with_exclusions_and_ssg(App, None, move || {
            provide_context(world_for_route_generation.clone())
        });
    build_static_routes_with_world(
        &leptos_options,
        App,
        world.clone(),
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

        let addr = leptos_options.site_addr.clone();
        println!("listening on http://{}", &addr);

        return actix_web::HttpServer::new(move || {
            let site_root = &leptos_options.site_root;

            actix_web::App::new()
                .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
                // serve JS/WASM/CSS from `pkg`
                .service(Files::new("/pkg", format!("{site_root}/pkg")))
                // serve other assets from the `assets` directory
                .service(Files::new("/assets", site_root))
                .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
                .app_data(web::Data::new(leptos_options.to_owned()))
        })
        .bind(&addr)?
        .run()
        .await;
    }
    #[cfg(not(feature = "development"))]
    Ok(())
}

pub async fn build_static_routes_with_world<IV>(
    options: &LeptosOptions,
    app_fn: impl Fn() -> IV + 'static + Clone,
    world: Arc<Mutex<World>>,
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
                boxed_system.initialize(&mut world);
                let params = boxed_system.run((), &mut world);
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
