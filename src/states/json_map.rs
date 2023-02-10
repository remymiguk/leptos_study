use crate::states::object::Object;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map};
use std::marker::PhantomData;
use voxi_core::{
    objects::value_json::{get_field_to_str, set_field_from_str},
    CoreError, ValueType,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JsonMap<T: Object> {
    #[serde(skip)]
    _phantom: PhantomData<T>,
    #[serde(flatten)]
    object: serde_json::Value,
}

impl<T: Object> JsonMap<T> {
    pub fn empty() -> JsonMap<T> {
        Self {
            object: json! { {} },
            _phantom: Default::default(),
        }
    }

    pub fn new(object: impl Serialize) -> Self {
        let object = serde_json::to_value(object).unwrap();
        Self {
            object,
            _phantom: Default::default(),
        }
    }

    pub fn try_from(object: T) -> Result<Self, CoreError> {
        let value = serde_json::to_value(&object)?;
        Ok(Self::new(value))
    }

    pub fn try_get(&self) -> Result<T, CoreError> {
        let object = serde_json::from_value(self.object.clone())?;
        Ok(object)
    }

    pub fn get(&self) -> T {
        self.try_get().unwrap()
    }

    pub fn get_value_str(
        &self,
        field_name: &str,
        value_type: ValueType,
    ) -> Result<Option<String>, CoreError> {
        let value = get_field_to_str(&self.object, field_name, value_type);
        Ok(value)
    }

    pub fn set_value_str(
        &mut self,
        field_name: &str,
        value_s: Option<String>,
        value_type: ValueType,
    ) -> Result<(), CoreError> {
        self.object = set_field_from_str(&self.object, field_name, value_s, value_type)?;
        Ok(())
    }

    pub fn object(&self) -> &serde_json::Value {
        &self.object
    }

    pub fn map(&self) -> &Map<String, serde_json::Value> {
        self.object.as_object().unwrap()
    }
}

impl<T: Object> From<T> for JsonMap<T> {
    fn from(value: T) -> Self {
        Self::try_from(value).unwrap()
    }
}

impl<T: Object> From<JsonMap<T>> for serde_json::Value {
    fn from(value: JsonMap<T>) -> Self {
        value.object
    }
}

// impl<T: Object> From<serde_json::Value> for JsonMap<T> {
//     fn from(value: T) -> Self {
//         Self::try_from(value).unwrap()
//     }
// }
