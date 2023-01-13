#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use axum::{extract::Extension, routing::get};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_study::file::file_handler;
    use leptos_study::routes::app::*;
    use std::sync::Arc;

    // The URL path of the generated JS/WASM bundle from cargo-leptos

    let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_address;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // build our application with a route
    let app = Router::new()
        .route("/favicon.ico", get(file_handler))
        .leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <App/> })
        .fallback(file_handler)
        .layer(Extension(Arc::new(leptos_options)));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// client-only stuff for Trunk
#[cfg(not(feature = "ssr"))]
pub fn main() {
    use leptos::*;
    use leptos_study::app::repository::*;
    use leptos_study::repositories::product::BufferProductRepository;
    use leptos_study::routes::app::*;

    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();

    set_product_repository(BufferProductRepository::new());

    mount_to_body(|cx| {
        view! { cx, <App/> }
    });
}
