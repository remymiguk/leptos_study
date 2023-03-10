use leptos::*;

// pub struct HoldOnSourceObject {}

// pub trait HoldOnSource<T> {
//     fn into_hold_on_source<V, F, E, C>(self, cx: Scope, fallback: F, error: E, child: C) -> View
//     where
//         F: Fn() -> V + 'static,
//         E: Fn() -> V + 'static,
//         C: Fn(T) -> V + 'static,
//         V: IntoView;
// }

// impl<T> HoldOnSource<T> for Option<Option<T>> {
//     fn into_hold_on_source<V, F, E, C>(self, cx: Scope, fallback: F, error: E, child: C) -> View
//     where
//         F: Fn() -> V + 'static,
//         E: Fn() -> V + 'static,
//         C: Fn(T) -> V + 'static,
//         V: IntoView,
//     {
//         let s = self;
//         {move ||
//                 match s {
//                     Some(result) => match result {
//                         Some(payload) => child(payload).into_view(cx),
//                         None => error().into_view(cx),
//                     },
//                     None => fallback().into_view(cx),
//                 }

//     }
//     .into_view(cx)
// }

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
    F: Fn(Scope) -> V + 'static,
    E: Fn(Scope) -> V + 'static,
    C: Fn(Scope, T) -> V + 'static,
    V: IntoView,
{
    move || match read() {
        Some(result) => match result {
            Some(payload) => child(cx, payload).into_view(cx),
            None => error(cx).into_view(cx),
        },
        None => fallback(cx).into_view(cx),
    }
}

#[component]
pub fn HoldOnCx<T, V, R, F, E, C>(
    cx: Scope,
    read: R,
    fallback: F,
    error: E,
    child: C,
) -> impl IntoView
where
    R: Fn() -> Option<Option<T>> + 'static,
    F: Fn(Scope) -> V + 'static,
    E: Fn(Scope) -> V + 'static,
    C: Fn(Scope, T) -> V + 'static,
    V: IntoView,
{
    move || match read() {
        Some(result) => match result {
            Some(payload) => child(cx, payload).into_view(cx),
            None => error(cx).into_view(cx),
        },
        None => fallback(cx).into_view(cx),
    }
}

#[component]
pub fn HoldOnRes<I, T, V, R, F, E, C>(
    cx: Scope,
    resource: R,
    fallback: F,
    error: E,
    child: C,
) -> impl IntoView
where
    I: Clone + 'static,
    T: Clone + 'static,
    R: Fn() -> Resource<I, Option<T>> + 'static,
    F: Fn(Scope) -> V + 'static,
    E: Fn(Scope) -> V + 'static,
    C: Fn(Scope, T) -> V + 'static,
    V: IntoView,
{
    move || match {
        let res = resource();
        (res.loading()(), res.read())
    } {
        (false, Some(result)) => match result {
            Some(payload) => child(cx, payload).into_view(cx),
            None => error(cx).into_view(cx),
        },
        _ => fallback(cx).into_view(cx),
    }
}

pub fn read_resource<T>(loading: bool, payload: Option<T>) -> Option<T> {
    match (loading, payload) {
        (false, Some(payload)) => Some(payload),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_resource_test() {
        assert_eq!(read_resource(false, None), Option::<usize>::None);
        assert_eq!(read_resource(false, Some(1)), Some(1));
        assert_eq!(read_resource(true, Some(1)), None);
        assert_eq!(read_resource(true, None), Option::<usize>::None);
    }
}
