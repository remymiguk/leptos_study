use crate::states::{
    app_state::{AppState, StateGetter, StateSetter, User},
    form_object::FormObject,
};
use leptos::*;
use serde::{Deserialize, Serialize};

// #[component]
// pub fn InputBind(cx: Scope, field_name: String) -> impl IntoView {
//     let content = memo_content_map(cx, field_name.clone(), ValueType::String);
//     let on_input = event_to_map(cx, field_name, ValueType::String);

//     view! {
//         cx,
//         <input class="input is-primary" type="text" placeholder="Primary input"
//         on:input=on_input
//         value=content/>
//     }
// }

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EmailPassword {
    email: String,
    password: String,
}

// pub trait ObjectFieldName: Serialize + DeserializeOwned {
//     fn value(&self, name: &str) -> Option<String> {
//         let j = serde_json::to_value(self).unwrap();
//         let map = j.as_object().unwrap();
//         map.get(name).map(|v| v.to_string())
//     }

//     fn set_value<T: Serialize + DeserializeOwned>(self, name: String, value: Option<String>) -> T {
//         let mut j = serde_json::to_value(self).unwrap();
//         let mut map = j.as_object_mut().unwrap();
//         todo!()
//     }
// }

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let email_password = EmailPassword {
        email: String::from("vanius@gmail.com"),
        password: String::from("password"),
    };

    let form_object = FormObject::new(cx, &email_password);
    let email_input = form_object.input_bind(cx, "email");
    let password_input = form_object.input_bind(cx, "password");

    let read_signal = form_object.read_signal();

    let on_click = move |_| {
        let app_state = use_context::<StateGetter<AppState>>(cx).unwrap().0;
        let set_app_state = use_context::<StateSetter<AppState>>(cx).unwrap().0;

        let email_password = read_signal().try_to::<EmailPassword>().unwrap();

        set_app_state(app_state().with_login(User {
            name: email_password.email.clone(),
            email: email_password.email,
        }));

        let navigator = window().history().unwrap();
        navigator.back().unwrap();
    };

    view! {
        cx,
            <div>
                <div>{move ||format!("{:?}", read_signal().object())}</div>
                <div>{ "User" }</div>
                { email_input }
                <div>{ "Password" }</div>
                { password_input }
                <br/>
                <br/>
                <input
                    class="button is-danger"
                    on:click=on_click
                    type="button"
                    value="Cancel"/>
            </div>

            <input
                class="button is-danger"
                on:click=on_click
                type="button"
                value="Login"/>
    }
    .into_view(cx)
}
