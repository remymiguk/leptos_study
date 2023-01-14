use crate::components::app_routes::*;
use crate::components::nav::*;
use crate::states::app_state::AppState;
use crate::states::app_state::StateGetter;
use crate::states::app_state::StateSetter;
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
                <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
                <Stylesheet id="leptos" href="./pkg/leptos_study.css"/>
                <Meta name="description" content="Leptos study app"/>

                // <Link href="https://fonts.googleapis.com/icon?family=Material+Icons" rel="stylesheet" />
    //           <Script src="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.2.1/js/all.min.js" />

                <header class="header">
                    <section class="hero is-primary">
                        <div class="hero-body">
                            <p class="title">{"Rust"}</p>
                            <p class="subtitle">{"Leptos study app"}</p>
                        </div>
                    </section>
                </header>

                <div class="columns is-mobile">
                    // is-half
                    <div class="column is-offset-one-quarter">
                        <Router>
                            <Nav />
                            <main>
                               <AppRoutes/>
                            </main>
                        </Router>
                    </div>
                </div>

                <footer class="footer">
                    <div class="content has-text-right">
                        <p><strong>{"Leptos"}</strong> {" powered by Rust"}</p>
                    </div>
                </footer>
        }
}
