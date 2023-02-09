use super::input_attributes::InputAttributes;
use voxi_core::ValueType;

pub struct InputValueType(pub InputAttributes, pub ValueType);

pub trait IntoInputValueType {
    fn into_input_value_type(self) -> InputValueType;
}

impl IntoInputValueType for InputValueType {
    fn into_input_value_type(self) -> InputValueType {
        self
    }
}
