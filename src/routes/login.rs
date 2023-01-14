use crate::states::{
    app_state::{AppState, LoggedUser, StateSetter},
    form_object::FormObject,
};
use leptos::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

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
pub fn InputBind<T, 'a>(cx: Scope, fo: &'a FormObject<T>, field_name: &'a str) -> impl IntoView
where
    T: Serialize + DeserializeOwned + Clone + 'static,
{
    view! {
        cx,
        <>
            { FormObject::input_bind(fo, field_name) }
        </>
    }
}

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let email_password = EmailPassword {
        email: Some(String::from("vanius@gmail.com")),
        password: Some(String::from("password")),
    };

    let fo = FormObject::new(cx, email_password);

    let (read_signal, write_signal) = fo.signal();

    view! {
        cx,
            <div>
                <div>{move ||format!("{:?}", read_signal().get())}</div>
                <div>{ "User" }</div>
                <InputBind fo=&fo field_name = "email"/>
                <div>{ "Password" }</div>
                <InputBind fo=&fo field_name = "password"/>
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
