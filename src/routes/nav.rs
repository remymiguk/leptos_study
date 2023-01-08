use leptos::*;
use leptos_router::*;

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    view! { cx,
        <>
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-menu">

                    <div class="navbar-start">

                        // FIXME: here if change to <A> the warning goes away
                        // <a href="/" class="navbar-item">
                        //     <strong>"Home"</strong>
                        // </a>
                        // <a href="/products" class="navbar-item">
                        //     <strong>"Product"</strong>
                        // </a>

                        <A href="/" class="navbar-item".to_string()>
                            <strong>"Home"</strong>
                        </A>
                        <A href="/products" class="navbar-item".to_string()>
                            <strong>"Product"</strong>
                        </A>


                    </div>

                </div>
            </nav>
        </>
    }
}
