use crate::routes::home::*;
use crate::routes::login::*;
use crate::routes::products::*;
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
            <Route path="settings" view=|cx| view! { cx,  <Settings/> }/>
            <Route path="products" view=|cx| view! { cx,  <Products/> }/>
            <Route path="product/:id" view=|cx| view! { cx,  <ProductForm/> }/>
        </Routes>
    }
}
