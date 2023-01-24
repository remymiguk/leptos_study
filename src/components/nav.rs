use crate::states::app_state::{AppState, StateGetter};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    let app_state = use_context::<StateGetter<AppState>>(cx).unwrap().0;
    let logged_menus = move || match app_state().logged_user() {
        Some(_) => view! {
        cx,
        <A href="/products" class="navbar-item".to_string()>
            <strong>"Product"</strong>
        </A>
        <A href="/settings" class="navbar-item".to_string()>
            <strong>"Settings"</strong>
        </A> }
        .into_view(cx),
        None => view! {
        cx,
        <A href="/login" class="navbar-item".to_string()>
            <strong>"Login"</strong>
        </A> }
        .into_view(cx),
    };

    view! { cx,
        <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <div class="navbar-start">


                    <A href="/" class="navbar-item".to_string()>
                        <strong>"Home"</strong>
                    </A>

                    <A href="/playground" class="navbar-item".to_string()>
                        <strong>"Playground"</strong>
                    </A>

                    {logged_menus}

                </div>
            </div>
        </nav>
    }
}
