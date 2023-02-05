use crate::components::modal::Confirmation;
use crate::states::checkbox_bind::*;
use crate::states::input_bind::*;
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
    let (read_signal, write_signal) = model.signal();

    let fo = FormObject::new(model);

    let confirm_clear = Confirmation::new(cx);

    view! {
        cx,
        <div>{move ||format!("Object content: {:?}", read_signal())}</div>
        <InputBind fo=&fo input_type="text" literal="User name" field_name="user_name" placeholder="User name"/>
        <InputBind fo=&fo input_type="email" literal="E-mail" field_name="email" placeholder="User e-mail"/>
        <InputBind fo=&fo input_type="password" literal="Password" field_name="password" placeholder="User password"/>
        <InputBind fo=&fo input_type="i64" literal="Code i64" field_name="code_i64" placeholder="Code i64"/>
        <InputBind fo=&fo input_type="u64" literal="Code u64" field_name="code_u64" placeholder="Code u64"/>
        <InputBind fo=&fo input_type="date" literal="Date" field_name="date" placeholder="Date"/>

        <CheckboxBind fo=&fo literal="Checkbox" field_name="checkbox"/>

        {confirm_clear.component(cx, "Confirm clear?", move |_| write_signal.set(Object::default()))}
        <input type="button" value="Clear" class="button is-danger"
            on:click=confirm_clear.on_show()
        />
    }
}
