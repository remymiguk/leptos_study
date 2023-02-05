use super::{
    input_attributes::InputAttributes,
    input_type::InputType,
    json_map::JsonMap,
    object_model::{ComponentMap, ObjectModel},
    validator::ValidatorProvider,
};
use crate::states::input_mode::InputMode;
use crate::states::object::Object;
use leptos::*;
use log::info;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use voxi_core::{objects::value_json::json_to_str, ValueType};
use web_sys::Event;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ComponentData {
    pub value: serde_json::Value,
    pub hint: Option<String>,
    pub valid: Option<bool>,
}

impl ComponentData {
    pub fn new(value: serde_json::Value) -> Self {
        Self {
            value,
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
    validators.retain(|v| v.trigger_field_name().name.name() == field_name);
    validators
}

#[derive(Clone)]
pub struct FormObject<T: Object> {
    object_read_signal: Memo<(JsonMap<T>, ComponentMap)>,
    object_writer_signal: WriteSignal<T>,
}

impl<T: Object> FormObject<T> {
    pub fn new(object_model: ObjectModel<T>) -> Self {
        let object_read_signal = object_model.public_component_read;
        let object_writer_signal = object_model.public_object_write;

        Self {
            object_read_signal,
            object_writer_signal,
        }
    }

    #[allow(unused_variables, clippy::too_many_arguments)]
    pub fn input_bind(
        &self,
        cx: Scope,
        value_type: ValueType,
        field_name: String,
        literal: String,
        input_attributes: InputAttributes,
    ) -> impl IntoView {
        let content_signal = self.memo_content(cx, field_name.clone(), value_type);
        let content_s = move || {
            let content_signal = content_signal();
            info!("content_signal: `{content_signal:?}`");
            content_signal.0
        };

        let is_valid_signal = self.memo_valid(cx, field_name.clone());
        let hint_signal = self.memo_hint(cx, field_name.clone());

        let on_input = self.event_to_map(field_name, value_type);

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

        let inputmode = input_attributes.inputmode.map(|im| im.to_string());

        let classes_div = move || {
            format!(
                "control has-icons-left has-icons-right {}",
                is_success_read().0
            )
        };
        let classes_input = move || format!("input {}", is_success_read().1);

        let min = input_attributes.min.map(|n| n.to_string());
        let max = input_attributes.max.map(|n| n.to_string());
        let step = input_attributes.step.map(|n| n.to_string());

        let input_type = input_attributes.input_type.to_string();

        view! {
            cx,
            <div class="field">
                <label class="label">{literal}</label>
                <div class={classes_div}>
                    <input class={classes_input} type={input_type}
                        {readonly} {disabled} {required} placeholder={input_attributes.placeholder}
                        inputmode={inputmode} {autofocus} {multiple} size={input_attributes.size} maxlength={input_attributes.maxlength}
                        min={min} max={max} pattern={input_attributes.pattern} width={input_attributes.width} height={input_attributes.height} step={step}
                        autocomplete={input_attributes.autocomplete}
                        on:input=on_input
                        prop:value=content_s
                    />
                </div>
                { hint_bottom }
            </div>
        }
    }

    fn memo_content(
        &self,
        cx: Scope,
        field_name: String,
        value_type: ValueType,
    ) -> Memo<(String, JsonMap<T>)> {
        let read_signal = self.object_read_signal;
        create_memo(cx, move |_| {
            let json_map = read_signal();
            let user_json = json_map.0;
            let value_j = json_map.1.map().get(&field_name).unwrap().value.clone();

            let value_s = json_to_str(value_j, value_type);
            info!(
                "inside memo content: `{field_name}` value: `{value_s}` user_json: `{user_json:?}`"
            );
            (value_s, user_json)
        })
    }

    fn memo_hint(&self, cx: Scope, field_name: String) -> Memo<Option<String>> {
        let read_signal = self.object_read_signal;
        create_memo(cx, move |_| {
            let json_map = read_signal();
            json_map.1.map().get(&field_name).unwrap().hint.clone()
        })
    }

    fn memo_valid(&self, cx: Scope, field_name: String) -> Memo<bool> {
        let read_signal = self.object_read_signal;
        create_memo(cx, move |_| -> bool {
            let json_map = read_signal();
            json_map
                .1
                .map()
                .get(&field_name)
                .unwrap()
                .valid
                .unwrap_or(true)
        })
    }

    fn event_to_map(&self, field_name: String, value_type: ValueType) -> impl Fn(Event) {
        let read_signal = self.object_read_signal;
        let write_signal = self.object_writer_signal;

        move |e: Event| {
            let value_s = event_target_value(&e);
            let mut form_map = JsonMap::new(read_signal.get().1.object());

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

pub struct InputValueType(pub InputAttributes, pub ValueType);

pub trait IntoInputValueType {
    fn into_input_value_type(self) -> InputValueType;
}

impl IntoInputValueType for InputValueType {
    fn into_input_value_type(self) -> InputValueType {
        self
    }
}

pub enum InputBindType {
    Uuid,
    Text,
    Password,
    Email,
    Decimal,
    I64,
    U64,
    Date,
    DateTime,
    Checkbox,
}

impl TryFrom<&str> for InputBindType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        let ib_type = match &value[..] {
            "uuid" => InputBindType::Uuid,
            "text" => InputBindType::Text,
            "password" => InputBindType::Password,
            "email" => InputBindType::Email,
            "decimal" => InputBindType::Decimal,
            "i64" => InputBindType::I64,
            "u64" => InputBindType::U64,
            "date" => InputBindType::Date,
            "datetime" => InputBindType::DateTime,
            "checkbox" => InputBindType::Checkbox,
            s => return Err(format!("undefined input bind type `{s}`")),
        };
        Ok(ib_type)
    }
}

impl IntoInputValueType for InputBindType {
    fn into_input_value_type(self) -> InputValueType {
        match self {
            InputBindType::Text => {
                InputValueType(InputAttributes::new(InputType::Text), ValueType::String)
            }
            InputBindType::Uuid => {
                InputValueType(InputAttributes::new(InputType::Text), ValueType::Uuid)
            }
            InputBindType::Password => {
                InputValueType(InputAttributes::new(InputType::Password), ValueType::String)
            }
            InputBindType::Decimal => InputValueType(
                InputAttributes::new(InputType::Number).step(dec!(0.01)),
                ValueType::Decimal,
            ),
            InputBindType::I64 => InputValueType(
                InputAttributes::new(InputType::Number).step(dec!(1)),
                ValueType::Int64,
            ),
            InputBindType::U64 => InputValueType(
                InputAttributes::new(InputType::Number)
                    .step(dec!(1))
                    .min(dec!(0)),
                ValueType::Int64,
            ),
            InputBindType::Date => {
                InputValueType(InputAttributes::new(InputType::Date), ValueType::Date)
            }
            InputBindType::DateTime => InputValueType(
                InputAttributes::new(InputType::DatetimeLocal),
                ValueType::DateTime,
            ),
            InputBindType::Checkbox => InputValueType(
                InputAttributes::new(InputType::Checkbox),
                ValueType::Boolean,
            ),
            InputBindType::Email => {
                InputValueType(InputAttributes::new(InputType::Email), ValueType::String)
            }
        }
    }
}

#[component]
pub fn InputBind<T, 'a>(
    cx: Scope,
    fo: &'a FormObject<T>,
    #[prop(into)] input_type: String,
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
    #[prop(optional)] min: Option<Decimal>,
    #[prop(optional)] max: Option<Decimal>,
    #[prop(optional)] pattern: Option<String>,
    #[prop(optional)] width: Option<usize>,
    #[prop(optional)] height: Option<usize>,
    #[prop(optional)] step: Option<Decimal>,
    #[prop(optional)] autocomplete: Option<String>,
) -> impl IntoView
where
    T: Object,
{
    let ibt: InputBindType = (&input_type[..]).try_into().unwrap();

    let InputValueType(mut input_attributes, value_type) = ibt.into_input_value_type();
    input_attributes.readonly = input_attributes.readonly.or(readonly);
    input_attributes.disabled = input_attributes.disabled.or(disabled);
    input_attributes.required = input_attributes.required.or(required);
    input_attributes.placeholder = input_attributes.placeholder.or(placeholder);
    input_attributes.inputmode = input_attributes.inputmode.or(inputmode);
    input_attributes.autofocus = input_attributes.autofocus.or(autofocus);
    input_attributes.multiple = input_attributes.multiple.or(multiple);
    input_attributes.size = input_attributes.size.or(size);
    input_attributes.maxlength = input_attributes.maxlength.or(maxlength);
    input_attributes.min = input_attributes.min.or(min);
    input_attributes.max = input_attributes.max.or(max);
    input_attributes.pattern = input_attributes.pattern.or(pattern);
    input_attributes.width = input_attributes.width.or(width);
    input_attributes.height = input_attributes.height.or(height);
    input_attributes.step = input_attributes.step.or(step);
    input_attributes.autocomplete = input_attributes.autocomplete.or(autocomplete);

    view! {
        cx,
        <>
            {fo.input_bind(cx, value_type,field_name, literal, input_attributes)}
        </>
    }
}

pub fn test() {
    let mut input_attributes = InputAttributes::default();

    //input_attributes.readonly = input_attributes.readonly.or(readonly);
}
