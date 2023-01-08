use crate::routes::home::*;
use crate::routes::nav::*;
use crate::routes::product_form::*;
use crate::routes::products::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    view! {
        cx,
        <>
            <Stylesheet id="leptos" href="./target/site/pkg/leptos_study.css"/>
            <Meta name="description" content="Leptos implementation of a HackerNews demo."/>

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
                                <Route path="products" view=|cx| view! { cx,  <Products/> }/>
                                <Route path="products/:id" view=|cx| view! { cx,  <ProductForm/> }/>
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
