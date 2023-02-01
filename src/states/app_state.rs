use leptos::{create_signal, provide_context, use_context, ReadSignal, Scope, WriteSignal};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoggedUser {
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppState {
    logged_user: Option<LoggedUser>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            logged_user: Default::default(),
        }
    }

    pub fn logged_user(&self) -> Option<LoggedUser> {
        self.logged_user.clone()
    }

    pub fn with_login(mut self, user: LoggedUser) -> Self {
        self.logged_user = Some(user);
        self
    }

    pub fn with_logoff(mut self) -> Self {
        self.logged_user = None;
        self
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Copy, Clone)]
pub struct StateSetter<T: 'static>(pub WriteSignal<T>);

#[derive(Copy, Clone)]
pub struct StateGetter<T: 'static>(pub ReadSignal<T>);

pub fn try_read_global_state<T, F>(cx: Scope, create: F) -> T
where
    T: Clone + 'static,
    F: Fn() -> T,
{
    use_context::<StateGetter<Option<T>>>(cx).unwrap().0().unwrap_or_else(create)
}

pub fn read_global_state<T: Clone + 'static>(cx: Scope) -> T {
    use_context::<StateGetter<Option<T>>>(cx).unwrap().0().unwrap()
}

pub fn write_global_state<T: Clone + 'static>(cx: Scope, model: T) {
    let model_write = use_context::<StateSetter<Option<T>>>(cx).unwrap();
    model_write
        .0
        .update(move |model_opt| *model_opt = Some(model));
}

pub fn declare_state<T: Clone + 'static>(cx: Scope) {
    let (product_model, set_product_model) = create_signal(cx, Option::<T>::None);
    provide_context(cx, StateSetter(set_product_model));
    provide_context(cx, StateGetter(product_model));
}
