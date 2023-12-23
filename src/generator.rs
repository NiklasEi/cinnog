use crate::DataLayer;
use axum::response::Response as AxumResponse;
use axum::{
    body::{boxed, Body, BoxBody},
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::IntoResponse,
};
use leptos::*;
use leptos_axum::generate_route_list_with_exclusions_and_ssg_and_context;
use leptos_router::{RouteListing, StaticDataMap, StaticParamsMap, StaticPath};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::task;
use tower::ServiceExt;
use tower_http::services::ServeDir;

impl DataLayer {
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

        let local = task::LocalSet::new();
        let app_fn_clone = app_fn.clone();
        let leptos_options_clone = leptos_options.clone();
        let routes_clone = routes.clone();
        local
            .run_until(async move {
                build_static_routes_with_world(
                    &leptos_options_clone,
                    app_fn_clone,
                    data.clone(),
                    &routes_clone,
                    &static_data_map,
                )
                .await
                .expect("Failed to build static routes")
            })
            .await;

        #[cfg(feature = "development")]
        {
            use axum::routing::post;
            use axum::Router;
            use leptos_axum::LeptosRoutes;

            let addr = leptos_options.site_addr;
            println!("listening on http://{}", &addr);

            let app = Router::new()
                .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
                .leptos_routes(&leptos_options, routes, app_fn)
                .fallback(file_and_error_handler)
                .with_state(leptos_options);

            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .expect("Failed to start development server");
        }
        Ok(())
    }
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
                let params = world.run_boxed(&mut boxed_system, ());
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

pub async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    _req: Request<Body>,
) -> AxumResponse {
    let root = options.site_root.clone();
    let res = get_static_file(uri.clone(), &root).await.unwrap();

    res.into_response()
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}
