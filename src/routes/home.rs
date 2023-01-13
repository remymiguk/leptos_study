use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <h1 class="title">
            {"Welcome sales App!"}
        </h1>
    }
}
