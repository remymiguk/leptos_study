use crate::app::state::{AppState, StateGetter, StateSetter, User};
use leptos::*;

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let on_click = move |_| {
        let app_state = use_context::<StateGetter<AppState>>(cx).unwrap().0;
        let set_app_state = use_context::<StateSetter<AppState>>(cx).unwrap().0;

        set_app_state(app_state().with_login(User {
            name: "Vanius".to_string(),
            email: "vanius@gmail.com".to_string(),
        }));

        let navigator = window().history().unwrap();
        navigator.back().unwrap();
    };

    view! {
        cx,
        <input
            class="button is-danger"
            on:click=on_click
            type="button"
            value="Login"/>

    }
}
