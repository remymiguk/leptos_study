use super::{
    input_attributes::InputAttributes,
    input_type::InputType,
    json_map::JsonMap,
    object_model::{ComponentMap, ObjectModel},
    validator::ValidatorProvider,
};
use crate::states::object::Object;
use leptos::*;
use log::info;
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

    pub fn memo_content(
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

    pub fn memo_hint(&self, cx: Scope, field_name: String) -> Memo<Option<String>> {
        let read_signal = self.object_read_signal;
        create_memo(cx, move |_| {
            let json_map = read_signal();
            json_map.1.map().get(&field_name).unwrap().hint.clone()
        })
    }

    pub fn memo_valid(&self, cx: Scope, field_name: String) -> Memo<bool> {
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

    pub fn on_change_to_map(&self, field_name: String, value_type: ValueType) -> impl Fn(Event) {
        let read_signal = self.object_read_signal;
        let write_signal = self.object_writer_signal;
        move |e: Event| {
            info!("*** inside event_to_map {} {:?}", field_name, value_type);
            let value_s = event_target_checked(&e).to_string();
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

    pub fn on_input_to_map(&self, field_name: String, value_type: ValueType) -> impl Fn(Event) {
        let read_signal = self.object_read_signal;
        let write_signal = self.object_writer_signal;

        move |e: Event| {
            info!("*** inside event_to_map {} {:?}", field_name, value_type);
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
                InputAttributes::new(InputType::Number)
                    .step(dec!(0.01))
                    .pattern(r"^\d+(?:\.\d{1,2})?$"),
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
            // InputBindType::Checkbox => InputValueType(
            //     InputAttributes::new(InputType::Checkbox),
            //     ValueType::Boolean,
            // ),
            InputBindType::Email => {
                InputValueType(InputAttributes::new(InputType::Email), ValueType::String)
            }
        }
    }
}
