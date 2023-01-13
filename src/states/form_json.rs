use serde::{de::DeserializeOwned, Serialize};
use voxi_core::{
    objects::value_json::{get_field_to_str, set_field_from_str},
    ValueType,
};

#[derive(Clone)]
pub struct FormJson {
    object: serde_json::Value,
}

impl FormJson {
    pub fn new(object: serde_json::Value) -> Self {
        Self { object }
    }

    pub fn try_from(object: impl Serialize) -> Result<Self, serde_json::Error> {
        Ok(Self::new(serde_json::to_value(&object)?))
    }

    pub fn try_to<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.object.clone())
    }

    pub fn get_value_str(
        &self,
        field_name: &str,
        value_type: ValueType,
    ) -> Result<Option<String>, serde_json::Error> {
        let value = get_field_to_str(&self.object, field_name, value_type);
        Ok(value)
    }

    pub fn set_value_str(
        &mut self,
        field_name: &str,
        value_s: Option<String>,
        value_type: ValueType,
    ) -> Result<(), serde_json::Error> {
        self.object = set_field_from_str(&self.object, field_name, value_s, value_type);
        Ok(())
    }

    pub fn object(&self) -> &serde_json::Value {
        &self.object
    }
}
