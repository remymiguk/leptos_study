use leptos::{ReadSignal, WriteSignal};
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
