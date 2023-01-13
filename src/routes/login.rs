use crate::app::state::{AppState, StateGetter, StateSetter, User};
use leptos::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use voxi_core::{
    objects::value_json::{get_field_to_str, set_field_from_str},
    ValueType,
};
use web_sys::Event;

#[derive(Clone)]
pub struct FormJson(serde_json::Value);

impl FormJson {
    pub fn try_from(object: impl Serialize) -> Result<Self, serde_json::Error> {
        Ok(Self(serde_json::to_value(&object)?))
    }

    pub fn try_to<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.0.clone())
    }

    pub fn get_value_str(
        &self,
        field_name: &str,
        value_type: ValueType,
    ) -> Result<Option<String>, serde_json::Error> {
        let value = get_field_to_str(&self.0, field_name, value_type);
        Ok(value)
    }

    pub fn set_value_str(
        &mut self,
        field_name: &str,
        value_s: Option<String>,
        value_type: ValueType,
    ) -> Result<(), serde_json::Error> {
        self.0 = set_field_from_str(&self.0, field_name, value_s, value_type);
        Ok(())
    }
}

#[component]
pub fn InputBind(cx: Scope, field_name: String) -> impl IntoView {
    let content = memo_content_map(cx, field_name.clone(), ValueType::String);
    let on_input = event_to_map(cx, field_name, ValueType::String);

    view! {
        cx,
        <input class="input is-primary" type="text" placeholder="Primary input"
        on:input=on_input
        value=content/>
    }
}

fn memo_content_map(cx: Scope, field_name: String, value_type: ValueType) -> Memo<Option<String>> {
    let form_state = use_context::<StateGetter<FormJson>>(cx).unwrap().0;
    create_memo(cx, move |_| {
        form_state().get_value_str(&field_name, value_type).unwrap()
    })
}

fn event_to_map(cx: Scope, field_name: String, value_type: ValueType) -> impl Fn(Event) {
    move |e: Event| {
        let form_state = use_context::<StateGetter<FormJson>>(cx).unwrap().0;
        let value_s = event_target_value(&e);
        let mut form_map = form_state();
        let value_s = if value_s.is_empty() {
            None
        } else {
            Some(value_s)
        };
        form_map
            .set_value_str(&field_name, value_s, value_type)
            .unwrap();
        let set_form_state = use_context::<StateSetter<FormJson>>(cx).unwrap().0;
        set_form_state.set(form_map);
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

pub fn create_json_context<T: Serialize + DeserializeOwned>(
    cx: Scope,
    object: &T,
) -> (ReadSignal<FormJson>, WriteSignal<FormJson>) {
    let (map_state, set_map_state) = create_signal(cx, FormJson::try_from(object).unwrap());

    provide_context(cx, StateGetter(map_state));
    provide_context(cx, StateSetter(set_map_state));
    (map_state, set_map_state)
}

pub struct FormBind {}

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let email_password = EmailPassword {
        email: String::from("vanius@gmail.com"),
        password: String::from("password"),
    };

    let (map_state, set_map_state) = create_json_context(cx, &email_password);

    let on_click = move |_| {
        let app_state = use_context::<StateGetter<AppState>>(cx).unwrap().0;
        let set_app_state = use_context::<StateSetter<AppState>>(cx).unwrap().0;

        let email_password = map_state().try_to::<EmailPassword>().unwrap();

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
    }
    .into_view(cx)
}
