use leptos::*;

#[component]
pub fn HoldOn<T, V, R, F, E, C>(
    cx: Scope,
    read: R,
    fallback: F,
    error: E,
    child: C,
) -> impl IntoView
where
    R: Fn() -> Option<Option<T>> + 'static,
    F: Fn() -> V + 'static,
    E: Fn() -> V + 'static,
    C: Fn(T) -> V + 'static,
    V: IntoView,
{
    move || match read() {
        Some(result) => match result {
            Some(payload) => child(payload).into_view(cx),
            None => error().into_view(cx),
        },
        None => fallback().into_view(cx),
    }
}
