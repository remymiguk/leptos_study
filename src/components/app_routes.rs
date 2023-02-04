use crate::components::product_form::*;
use crate::routes::home::*;
use crate::routes::login::*;
use crate::routes::playground::*;
use crate::routes::product_table::*;
use crate::routes::settings::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn AppRoutes(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Routes>
            <Route path="/" view=|cx| view! { cx,  <Home/> }/>
            <Route path="login" view=|cx| view! { cx,  <Login/> }/>
            <Route path="playground" view=|cx| view! { cx,  <Playground/> }/>
            <Route path="settings" view=|cx| view! { cx,  <Settings/> }/>
            <Route path="product/:id" view=|cx| view! { cx,  <ProductForm/> }/>
            <Route path="products" view=|cx| view! { cx,  <ProductTable/> }/>
        </Routes>
    }
}
