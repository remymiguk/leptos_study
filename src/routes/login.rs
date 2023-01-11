use crate::app::state::{AppState, StateGetter, StateSetter, User};
use leptos::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
// use voxi_core::{objects::value_json::get_field_to_str, ValueType};
use web_sys::Event;

#[derive(Clone)]
pub struct FormMap(pub HashMap<String, String>);

// #[derive(Clone)]
// pub struct FormObject<T: Serialize + DeserializeOwned + Clone>(pub T);

#[component]
pub fn InputBind(cx: Scope, field_name: String) -> impl IntoView {
    let content = memo_content_map(cx, field_name.clone());
    let on_input = event_to_map(cx, field_name);

    view! {
        cx,
        <input class="input is-primary" type="text" placeholder="Primary input"
        on:input=on_input
        value=content/>
    }
}

fn memo_content_map(cx: Scope, field_name: String) -> Memo<String> {
    let form_state = use_context::<StateGetter<FormMap>>(cx).unwrap().0;
    create_memo(cx, move |_| {
        form_state().0.get(&field_name).cloned().unwrap_or_default()
    })
}

// fn memo_content_object<T: Serialize + DeserializeOwned + Clone + 'static>(
//     cx: Scope,
//     field_name: String,
//     value_type: ValueType,
// ) -> Memo<String> {
//     let form_state = use_context::<StateGetter<FormObject<T>>>(cx).unwrap().0;
//     create_memo(cx, move |_| {
//         let t = form_state().0;
//         get_field_to_str(&t, &field_name, &value_type).unwrap_or_default()
//     })
// }

fn event_to_map(cx: Scope, field_name: String) -> impl Fn(Event) {
    move |e: Event| {
        let form_state = use_context::<StateGetter<FormMap>>(cx).unwrap().0;
        let value_s = event_target_value(&e);
        let mut form_map = form_state().0;
        form_map.insert(field_name.clone(), value_s);
        let set_form_state = use_context::<StateSetter<FormMap>>(cx).unwrap().0;
        set_form_state.set(FormMap(form_map));
    }
}

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

pub struct FormBind {}

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    if true {
        return view! {
            cx,
            <div>
            </div>
        }
        .into_view(cx);
    }

    let email_password = EmailPassword {
        email: String::from("vanius@gmail.com"),
        password: String::from("password"),
    };

    let mut user_map = HashMap::new();
    user_map.insert(String::from("user"), String::from("vanius@gmail.com"));
    user_map.insert(String::from("password"), String::from(""));
    let (map_state, set_map_state) = create_signal(cx, FormMap(user_map));

    provide_context(cx, StateGetter(map_state));
    provide_context(cx, StateSetter(set_map_state));

    let on_click = move |_| {
        let app_state = use_context::<StateGetter<AppState>>(cx).unwrap().0;
        let set_app_state = use_context::<StateSetter<AppState>>(cx).unwrap().0;

        set_app_state(app_state().with_login(User {
            name: "Vanius".to_string(),
            email: "vanius@gmail.com".to_string(),
        }));

        let navigator = window().history().unwrap();
        navigator.back().unwrap();
    };

    view! {
        cx,
        <>

            <div>
                <div>{move ||format!("{:?}", map_state().0)}</div>
                <div>{ "User" }</div>
                <InputBind field_name="user".to_string()/>
                <div>{ "Password" }</div>
                <InputBind field_name="password".to_string()/>
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
        </>

    }
    .into_view(cx)
}