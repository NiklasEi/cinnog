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
use leptos_router::build_static_routes_with_additional_context;
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
                build_static_routes_with_additional_context(
                    &leptos_options_clone,
                    app_fn_clone,
                    move || provide_context(data.clone()),
                    &routes_clone,
                    &static_data_map,
                )
                .await
                .expect("Failed to build static routes")
            })
            .await;

        #[cfg(feature = "development")]
        {
            use axum::Router;

            let addr = leptos_options.site_addr;
            println!("listening on http://{}", &addr);

            let app = Router::new()
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
