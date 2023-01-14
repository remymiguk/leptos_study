use crate::states::{
    app_state::{AppState, LoggedUser, StateSetter},
    form_object::*,
};
use leptos::*;
use serde::{Deserialize, Serialize};

pub fn apply_login(cx: Scope, email_password: EmailPassword) {
    let set_app_state = use_context::<StateSetter<AppState>>(cx).unwrap().0;

    set_app_state.update(move |app_state| {
        *app_state = app_state.clone().with_login(LoggedUser {
            name: email_password.email.clone().unwrap_or_default(),
            email: email_password.email.unwrap_or_default(),
        })
    });
}

// Where is the typing event?
// Where are the fields are being cleaned?

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct EmailPassword {
    email: Option<String>,
    password: Option<String>,
}

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let fo = FormObject::new(cx, EmailPassword::default());
    let (read_signal, write_signal) = fo.signal();
    view! {
        cx,
            <div>{move ||format!("Object content: {:?}", read_signal().get())}</div>
            <div>{"User"}</div>
            <InputBind fo=&fo field_name="email"/>
            <div>{"Password"}</div>
            <InputBind fo=&fo field_name="password"/>
            <br/><br/>
            <input type="button" value="Login" class="button is-danger"
                on:click=move |_| {
                    apply_login(cx, read_signal().get());
                    history_back();
                }
            />
            <br/><br/>
            <input type="button" value="Clear" class="button is-danger"
                on:click=move |_| write_signal.set(EmailPassword::default().into())
            />

    }
}

fn history_back() {
    window().history().unwrap().back().unwrap()
}
