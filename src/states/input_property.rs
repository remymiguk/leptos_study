use super::{
    input_attributes::InputAttributes, input_icon::InputIcon, validator::ValidatorProvider,
};
use voxi_core::ValueType;

#[derive(Clone)]
pub struct InputProperty {
    pub field_name: String,
    pub literal: String,
    // pub map_state: Box<dyn MapStateTrait>,
    pub icon: Option<InputIcon>,
    pub validator: Option<Box<dyn ValidatorProvider>>,
    pub input_attributes: InputAttributes,
    pub value_type: ValueType,
}
