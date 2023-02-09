use rust_decimal_macros::dec;
use voxi_core::ValueType;

use super::{
    input_attributes::InputAttributes,
    input_type::InputType,
    input_validator::DecimalValidator,
    input_value_type::{InputValueType, IntoInputValueType},
};

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
                    .validator(DecimalValidator::new(2))
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
