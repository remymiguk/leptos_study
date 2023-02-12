pub mod utils;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use axum::{extract::Extension, routing::*};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_study::app::repository::add_repository_provider;
    use leptos_study::file::file_and_error_handler;
    use leptos_study::repositories::product::*;
    use leptos_study::routes::app::*;
    use std::sync::Arc;

    add_repository_provider(BufferProductRepositoryProvider::new()); // ApiProductRepository

    // The URL path of the generated JS/WASM bundle from cargo-leptos

    let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");

    // build our application with a route
    let app = Router::new()
        .route("/favicon.ico", get(file_and_error_handler))
        .route("/api/domain", patch(reader))
        .route("/api/update", patch(writer))

        .leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <App/> })
        .fallback(file_and_error_handler)
        .layer(Extension(Arc::new(leptos_options)));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "ssr")]
use voxi_dataset::domain::reader_request::ReaderRequest;
#[cfg(feature = "ssr")]
use voxi_dataset::domain::reader_response::ReaderResponse;
#[cfg(feature = "ssr")]
use voxi_dataset::writer::writer_request::WriterRequest;
#[cfg(feature = "ssr")]
use axum::Json;
#[cfg(feature = "ssr")]
use voxi_dataset::domain::domain_data_set_factory::reader_response_by_request;
#[cfg(feature = "ssr")]
use voxi_dataset::domain::domain_data_set_factory::write_response_by_request;

#[cfg(feature = "ssr")]
#[axum::debug_handler]
async fn reader(Json(request): Json<ReaderRequest>) -> Json<ReaderResponse> {

    // tracing::debug!(
    //     "domain request: {}",
    //     serde_json::to_string(&request).unwrap()
    // );
    let response = reader_response_by_request(request).await.unwrap();
    // tracing::debug!(
    //     "domain response: {}",
    //     serde_json::to_string(&response).unwrap()
    // );
    Json(response)
}

/// For update should receive a response
/// Is master_relation the context? I think so
///
#[cfg(feature = "ssr")]
#[axum::debug_handler]
async fn writer(Json(payload): Json<WriterRequest>) -> Json<ReaderResponse> {
    let response = write_response_by_request(payload).await.unwrap();
    Json(response)
}

// client-only stuff for Trunk
#[cfg(not(feature = "ssr"))]
pub fn main() {
    use leptos::*;
    use leptos_study::app::repository::*;
    use leptos_study::repositories::product::*;
    use leptos_study::routes::app::*;

    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();

    add_repository_provider(BufferProductRepositoryProvider::new()); // ApiProductRepository

    mount_to_body(|cx| {
        view! { cx, <App/> }
    });
}
