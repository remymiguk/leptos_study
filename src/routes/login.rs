use crate::states::{
    app_state::{AppState, LoggedUser, StateSetter},
    form_object::FormObject,
};
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct EmailPassword {
    email: Option<String>,
    password: Option<String>,
}

pub fn apply_login(cx: Scope, email_password: EmailPassword) {
    let set_app_state = use_context::<StateSetter<AppState>>(cx).unwrap().0;

    set_app_state.update(move |app_state| {
        *app_state = app_state.clone().with_login(LoggedUser {
            name: email_password.email.clone().unwrap_or_default(),
            email: email_password.email.unwrap_or_default(),
        })
    });
}

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let email_password = EmailPassword {
        email: Some(String::from("vanius@gmail.com")),
        password: Some(String::from("password")),
    };

    let form_object = FormObject::new(cx, email_password);

    let (read_signal, write_signal) = form_object.signal();

    view! {
        cx,
            <div>
                <div>{move ||format!("{:?}", read_signal().get())}</div>
                <div>{ "User" }</div>
                { form_object.input_bind("email") }
                <div>{ "Password" }</div>
                { form_object.input_bind("password") }
                <br/>
                <br/>
                <input
                    class="button is-danger"
                    on:click=move |_| {
                        apply_login(cx, read_signal().get());
                        history_back();
                    }   
                    type="button"
                    value="Login"/>
                <br/>
                <input
                    class="button is-danger"
                    on:click=move |_| write_signal.set(EmailPassword::default().into())
                    type="button"
                    value="Clear"/>
            </div>


    }
}

fn history_back() {
    window().history().unwrap().back().unwrap()
}
