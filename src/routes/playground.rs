use crate::states::{form_object::*, object_model::ObjectModel};
use chrono::NaiveDate;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Object {
    user_name: Option<String>,
    email: Option<String>,
    password: Option<String>,
    code_i64: Option<i64>,
    code_u64: Option<u64>,
    date: Option<NaiveDate>,
    checkbox: Option<bool>,
    select: Option<i32>,
    textarea: Option<String>,
}

#[component]
pub fn Playground(cx: Scope) -> impl IntoView {
    let model = ObjectModel::new(cx, Object::default(), vec![]);
    let fo = FormObject::new(model);

    view! {
        cx,
        <InputBind fo=&fo input_type="text" literal="User name" field_name="user_name" placeholder="User name"/>
        <InputBind fo=&fo input_type="text" literal="E-mail" field_name="email" placeholder="User e-mail"/>
        <InputBind fo=&fo input_type="password" literal="Password" field_name="password" placeholder="User password"/>

        <InputBind fo=&fo input_type="i64" literal="Code i64" field_name="code_i64" placeholder="Code i64"/>

        <InputBind fo=&fo input_type="u64" literal="Code u64" field_name="code_u64" placeholder="Code u64"/>

        <InputBind fo=&fo input_type="date" literal="Date" field_name="date" placeholder="Date"/>

            // <h4>
            //     {move || upsert_value}
            // </h4>
            // <br/><br/>
            // <input type="button" value="Signal" class="button is-danger"
            //     on:click=on_signal
            // />
            // <input type="button" value="Action" class="button is-danger"
            //     on:click=on_action
            // />
    }
}

async fn switch(payload: String) -> String {
    if payload == "..." {
        String::from("xxx")
    } else {
        String::from("...")
    }
}
