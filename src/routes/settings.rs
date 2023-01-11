use crate::app::state::{AppState, StateGetter, StateSetter};
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};

#[component]
pub fn Settings(cx: Scope) -> impl IntoView {
    let app_state = use_context::<StateGetter<AppState>>(cx).unwrap().0;
    let logged_user_s = app_state()
        .logged_user()
        .map(|u| format!("{} | {}", u.name, u.email))
        .unwrap_or_default();

    let on_click = move |_| {
        let set_app_state = use_context::<StateSetter<AppState>>(cx).unwrap().0;

        set_app_state(app_state().with_logoff());

        let go_to = use_navigate(cx);
        go_to("/", NavigateOptions::default()).unwrap();
    };

    view! {
        cx,
        <>
            <h4>{logged_user_s}</h4>
            <input
                class="button is-danger"
                on:click=on_click
                type="button"
                value="Logoff"/>
        </>

    }
}
