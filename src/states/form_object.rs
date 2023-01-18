use super::{
    json_map::JsonMap,
    object_model::{ComponentMap, ObjectModel},
    validator::ValidatorProvider,
};
use crate::states::input_mode::InputMode;
use crate::states::object::Object;
use leptos::*;
use log::info;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use voxi_core::{json_to_value, ValueType};
use web_sys::Event;

#[derive(Clone)]
pub struct ComponentAtts {
    literal: String,
    field_name: String,
    v_type: ValueType,
    data: ComponentData,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ComponentData {
    pub value: serde_json::Value,
    pub hint: Option<String>,
    pub valid: bool,
}

impl ComponentData {
    pub fn new(value: serde_json::Value) -> Self {
        Self {
            value,
            valid: true,
            ..Default::default()
        }
    }
}

impl From<serde_json::Value> for ComponentData {
    fn from(value: serde_json::Value) -> Self {
        Self::new(value)
    }
}

pub fn validators_by_field(
    mut validators: Vec<Box<dyn ValidatorProvider + 'static + Send + Sync>>,
    field_name: &str,
) -> Vec<Box<dyn ValidatorProvider + 'static + Send + Sync>> {
    validators.retain(|v| v.field_name().name.name() == field_name);
    validators
}

#[derive(Clone)]
pub struct FormObject<T: Object> {
    object_read_signal: Memo<ComponentMap>,
    _phantom_data: PhantomData<T>,
    object_writer_signal: WriteSignal<T>,
}

impl<T: Object> FormObject<T> {
    pub fn new(cx: Scope, object_model: ObjectModel<T>) -> Self {
        let object_read_signal = object_model.public_component_reader;
        let object_writer_signal = object_model.public_object_writer;

        Self {
            object_read_signal,
            object_writer_signal,
            _phantom_data: Default::default(),
        }
    }

    #[allow(unused_variables, clippy::too_many_arguments)]
    pub fn input_bind_text(
        &self,
        cx: Scope,
        literal: String,
        field_name: String,
        readonly: Option<bool>,
        disabled: Option<bool>,
        required: Option<bool>,
        placeholder: Option<String>,
        inputmode: Option<InputMode>,
        autofocus: Option<bool>,
        multiple: Option<bool>,
        size: Option<usize>,
        maxlength: Option<usize>,
        min: Option<usize>,
        max: Option<usize>,
        pattern: Option<String>,
        width: Option<usize>,
        height: Option<usize>,
        step: Option<usize>,
    ) -> impl IntoView {
        let content_signal = self.memo_content(cx, field_name.clone(), ValueType::String);
        let is_valid_signal = self.memo_valid(cx, field_name.clone());
        let hint_signal = self.memo_hint(cx, field_name.clone());

        let on_input = self.event_to_map(cx, field_name, ValueType::String);

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
                    <p class="help {is_success}">{hint}</p>
                }
                .into_view(cx)
            })
        });

        let inputmode = inputmode.map(|im| im.to_string());

        let classes_div = move || {
            format!(
                "control has-icons-left has-icons-right {}",
                is_success_read().0
            )
        };
        let classes_input = move || format!("input {}", is_success_read().1);

        view! {
            cx,
            <div class="field">
                <label class="label">{literal}</label>
                <div class={classes_div}>
                    <input class={classes_input} type="text"
                        {readonly} {disabled} {required} placeholder={placeholder}
                        inputmode={inputmode} {autofocus} {multiple} size={size} maxlength={maxlength}
                        min={min} max={max} pattern={pattern} width={width} height={height} step={step}
                        on:input=on_input
                        prop:value=content_signal
                    />
                </div>
                { hint_bottom }
            </div>
        }
    }

    fn memo_content(&self, cx: Scope, field_name: String, value_type: ValueType) -> Memo<String> {
        let read_signal = self.object_read_signal;
        create_memo(cx, move |_| {
            let json_map = read_signal();
            let value_j = json_map.map().get(&field_name).unwrap().value.clone();
            // TODO: create function json_to_str
            let nullable = json_to_value(value_j, value_type).unwrap();
            let value_s = nullable
                .into_opt()
                .map(|v| v.to_string())
                .unwrap_or_default();
            info!("inside memo field `{field_name}` value `{value_s}`");
            value_s
        })
    }

    fn memo_hint(&self, cx: Scope, field_name: String) -> Memo<Option<String>> {
        let read_signal = self.object_read_signal;
        create_memo(cx, move |_| {
            let json_map = read_signal();
            json_map.map().get(&field_name).unwrap().hint.clone()
        })
    }

    fn memo_valid(&self, cx: Scope, field_name: String) -> Memo<bool> {
        let read_signal = self.object_read_signal;
        create_memo(cx, move |_| {
            let json_map = read_signal();
            json_map.map().get(&field_name).unwrap().valid
        })
    }

    fn event_to_map(&self, cx: Scope, field_name: String, value_type: ValueType) -> impl Fn(Event) {
        let read_signal = self.object_read_signal;
        let write_signal = self.object_writer_signal;

        move |e: Event| {
            let value_s = event_target_value(&e);
            let mut form_map = JsonMap::new(read_signal.get().object());

            let value_s = if value_s.is_empty() {
                None
            } else {
                Some(value_s)
            };
            form_map
                .set_value_str(&field_name, value_s, value_type)
                .unwrap();
            let object: T = form_map.get();

            info!("*** firing {object:?}");
            write_signal.set(object);
        }
    }
}

#[component]
pub fn InputBind<T, 'a>(
    cx: Scope,
    fo: &'a FormObject<T>,
    #[prop(into)] literal: String,
    #[prop(into)] field_name: String,
    #[prop(optional)] readonly: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] required: Option<bool>,
    #[prop(into)]
    #[prop(optional)]
    placeholder: Option<String>,
    #[prop(optional)] inputmode: Option<InputMode>,
    #[prop(optional)] autofocus: Option<bool>,
    #[prop(optional)] multiple: Option<bool>,
    #[prop(optional)] size: Option<usize>,
    #[prop(optional)] maxlength: Option<usize>,
    #[prop(optional)] min: Option<usize>,
    #[prop(optional)] max: Option<usize>,
    #[prop(optional)] pattern: Option<String>,
    #[prop(optional)] width: Option<usize>,
    #[prop(optional)] height: Option<usize>,
    #[prop(optional)] step: Option<usize>,
) -> impl IntoView
where
    T: Object,
{
    view! {
        cx,
        <>
            {fo.input_bind_text(cx, literal, field_name, readonly, disabled, required, placeholder, inputmode,
                autofocus, multiple, size, maxlength, min, max, pattern, width, height, step,
            )}
        </>
    }
}
