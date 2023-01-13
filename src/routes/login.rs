use crate::states::{
    app_state::{AppState, LoggedUser, StateSetter},
    form_object::FormObject,
};
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct EmailPassword {
    email: String,
    password: String,
}

pub fn apply_login(cx: Scope, email_password: EmailPassword) {
    let set_app_state = use_context::<StateSetter<AppState>>(cx).unwrap().0;

    set_app_state.update(move |app_state| {
        *app_state = app_state.clone().with_login(LoggedUser {
            name: email_password.email.clone(),
            email: email_password.email,
        })
    });
}

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let email_password = EmailPassword {
        email: String::from("vanius@gmail.com"),
        password: String::from("password"),
    };

    let form_object = FormObject::new(cx, email_password);

    let read_signal = form_object.read_signal();
    let write_signal = form_object.write_signal();

    let on_login_click = move |_| {
        let email_password = read_signal().get();
        apply_login(cx, email_password);

        let navigator = window().history().unwrap();
        navigator.back().unwrap();
    };

    let on_login_clear = move |_| write_signal.set(EmailPassword::default().into());

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
                    on:click=on_login_clear
                    type="button"
                    value="Clear"/>
            </div>

            <input
                class="button is-danger"
                on:click=on_login_click
                type="button"
                value="Login"/>
    }
    .into_view(cx)
}
