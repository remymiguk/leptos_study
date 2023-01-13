use crate::app::state::AppState;
use crate::app::state::StateGetter;
use crate::app::state::StateSetter;
use crate::routes::home::*;
use crate::routes::login::*;
use crate::routes::nav::*;
use crate::routes::product_form::*;
use crate::routes::products::*;
use crate::routes::settings::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    let (app_state, set_app_state) = create_signal(cx, AppState::default());

    provide_context(cx, StateSetter(set_app_state));
    provide_context(cx, StateGetter(app_state));

    view! {
        cx,
        <>
            <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
            <Stylesheet id="leptos" href="./pkg/leptos_study.css"/>
            <Meta name="description" content="Leptos study app"/>

            <header class="header">
                <section class="hero is-primary">
                    <div class="hero-body">
                        <p class="title">{"Rust"}</p>
                        <p class="subtitle">{"Leptos study app"}</p>
                    </div>
                </section>
            </header>

            <div class="columns is-mobile">
                <div class="column is-half is-offset-one-quarter">
                    <Router>
                        <Nav />
                        <main>
                            <Routes>
                                <Route path="/" view=|cx| view! { cx,  <Home/> }/>
                                <Route path="login" view=|cx| view! { cx,  <Login/> }/>
                                <Route path="settings" view=|cx| view! { cx,  <Settings/> }/>
                                <Route path="products" view=|cx| view! { cx,  <Products/> }/>
                                <Route path="product/:id" view=|cx| view! { cx,  <ProductForm/> }/>
                            </Routes>
                        </main>
                    </Router>
                </div>
            </div>

            <footer class="footer">
                <div class="content has-text-right">
                    <p><strong>{"Leptos"}</strong> {" powered by Rust"}</p>
                </div>
            </footer>
        </>
    }
}
