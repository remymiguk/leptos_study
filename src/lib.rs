mod api;
pub mod app;
pub mod models;
pub mod routes;
pub mod repositories;
pub mod handlers;
pub mod file;
pub mod components;

// Needs to be in lib.rs AFAIK because wasm-bindgen needs us to be compiling a lib. I may be wrong.

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::repository::*;
    use crate::repositories::product::BufferProductRepository;
    use crate::routes::app::*;
    use leptos::view;

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    set_product_repository(BufferProductRepository::new());

    leptos::mount_to_body(move |cx| {
        view! { cx, <App/> }
    });
}
