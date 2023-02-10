use crate::states::form_object::FormObject;
use crate::states::object::Object;
use leptos::*;
use voxi_core::{IntoValue, IntoValueType};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectOption<T: IntoValue + Clone + std::fmt::Display> {
    value: T,
    literal: String,
}

pub trait IntoSelectOption<T: IntoValue + Clone + std::fmt::Display> {
    fn into_select_option(self) -> SelectOption<T>;
}

impl<T> IntoSelectOption<T> for (T, &str)
where
    T: IntoValue + Clone + std::fmt::Display,
{
    fn into_select_option(self) -> SelectOption<T> {
        SelectOption {
            value: self.0,
            literal: self.1.to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectOptions<T: IntoValue + Clone + std::fmt::Display> {
    options: Vec<SelectOption<T>>,
}

pub trait IntoSelectOptions<T: IntoValue + Clone + std::fmt::Display> {
    fn into_select_options(self) -> SelectOptions<T>;
}

impl<V, T> IntoSelectOptions<V> for Vec<T>
where
    V: IntoValue + Clone + std::fmt::Display,
    T: IntoSelectOption<V>,
{
    fn into_select_options(self) -> SelectOptions<V> {
        let options = self
            .into_iter()
            .map(|o| o.into_select_option())
            .collect::<Vec<_>>();
        SelectOptions { options }
    }
}

#[component]
pub fn SelectBind<T, V, 'a>(
    cx: Scope,
    fo: &'a FormObject<T>,
    #[prop(into)] literal: String,
    #[prop(into)] field_name: String,
    options: SelectOptions<V>,
) -> impl IntoView
where
    T: Object,
    V: IntoValue + Clone + std::fmt::Display + 'static,
{
    let value_type = options
        .options
        .first()
        .map(|v| v.value.clone().into_value())
        .unwrap()
        .into_value()
        .value_type();

    let onchange = fo.on_change_value_to_map(field_name.clone(), value_type);

    let current_value = fo.memo_content(cx, field_name.clone(), value_type)().1;
    let current_value_s = current_value.map(|v| v.to_string()).unwrap_or_default();

    let is_valid_signal = fo.memo_valid(cx, field_name.clone());
    let hint_signal = fo.memo_hint(cx, field_name);

    let is_success_read = create_memo(cx, move |_| {
        if is_valid_signal() {
            ("is-success", "fa-check")
        } else {
            ("is-danger", "fa-exclamation-triangle")
        }
    });

    let hint_bottom = create_memo(cx, move |_| {
        hint_signal().map(|hint| {
            let is_success = is_success_read().0;
            view! {
                cx,
                <p class={format!("help {is_success}")}>{hint}</p>
            }
            .into_view(cx)
        })
    });

    let options = options
        .options
        .into_iter()
        .map(|o| (o.value.to_string(), o.literal))
        .map(|(value_s, literal)| {
            let selected = value_s == current_value_s;
            view! {
                cx,
                <option value=value_s selected=selected>{ literal }</option>
            }
        })
        .collect::<Vec<_>>();

    let select = view! {
        cx,
        <div class="select is-success">
            <select on:change=onchange>
                { options }
            </select>
        </div>
    };

    view! {
        cx,
        <div class="field">
            <label class="label">{ literal }</label>
            <div class="control has-icons-left has-icons-right">
                { select }
                // { icon_left }
                // { icon_right }
            </div>
            { hint_bottom }
        </div>

    }
}
