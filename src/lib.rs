use leptos::{component, view, IntoView, Scope};
use leptos_meta::*;
use leptos_router::*;
mod api;
pub mod handlers;
pub mod models;
mod routes;
use routes::nav::*;
use routes::product_form::*;
use routes::products::*;
use routes::stories::*;
use routes::story::*;
use routes::users::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    view! {
        cx,
        <>
            <Stylesheet id="leptos" href="./target/site/pkg/hackernews_axum.css"/>
            <Meta name="description" content="Leptos implementation of a HackerNews demo."/>
            <Router>
                <Nav />
                <main>
                    <Routes>
                        //<Route path="/" view=|cx| view! { cx,  <Stories/> }/>
                        // <Route path="new" view=|cx| view! { cx,  <Stories/> }/>
                        // <Route path="show" view=|cx| view! { cx,  <Stories/> }/>
                        // <Route path="ask" view=|cx| view! { cx,  <Stories/> }/>
                        // <Route path="job" view=|cx| view! { cx,  <Stories/> }/>
                        <Route path="products" view=|cx| view! { cx,  <Products/> }/>
                        <Route path="users/:id" view=|cx| view! { cx,  <User/> }/>
                        <Route path="stories/:id" view=|cx| view! { cx,  <Story/> }/>
                        <Route path="products/:id" view=|cx| view! { cx,  <ProductForm/> }/>
                        <Route path="*stories" view=|cx| view! { cx,  <Stories/> }/>
                    </Routes>
                </main>
            </Router>
        </>
    }
}

// Needs to be in lib.rs AFAIK because wasm-bindgen needs us to be compiling a lib. I may be wrong.
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::routes::product_form::ProductForm;

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount_to_body(move |cx| {
        view! { cx, <App/> }
    });
}
