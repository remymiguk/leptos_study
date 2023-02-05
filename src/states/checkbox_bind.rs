use crate::states::form_object::IntoInputValueType;
use crate::states::object::Object;
use crate::states::{
    form_object::{FormObject, InputBindType, InputValueType},
    input_mode::InputMode,
};
use leptos::*;
use log::info;
use rust_decimal::Decimal;

#[component]
pub fn InputCheckbox<T, 'a>(cx: Scope, fo: &'a FormObject<T>) -> impl IntoView
where
    T: Object,
{
    view! {
        cx,
        <div class="field">
            <div class="control has-icons-left has-icons-right">
                { checkbox }
                { icon_left }
                { icon_right }
            </div>
            { hint_bottom }
        </div>

    }
}
