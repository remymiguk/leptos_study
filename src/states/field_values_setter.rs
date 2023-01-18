use super::validator::ValidatorProvider;
use voxi_core::ValueType;

#[derive(Clone)]
pub struct FieldValueSetter {
    pub trigger_field_name: String,
    // pub map_state: Box<dyn MapStateTrait>,
    pub validator: Option<Box<dyn ValidatorProvider>>,
    pub value_type: ValueType,
    // pub hint_state: UseStateHandle<String>,
    // pub success_state: UseStateHandle<bool>,
    pub value_s: String,
}
