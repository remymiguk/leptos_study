use super::{
    form_object::ComponentData,
    json_map::JsonMap,
    validator::{exec_validator, ValidatorProvider},
};
use crate::states::{form_object::validators_by_field, object::Object};
use leptos::*;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use voxi_core::{
    objects::value_json::{get_field_to_str, modified_fields_name},
    ValueType,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct JsonChanged<T: Object> {
    json_map: JsonMap<T>,
    fields_diff: Vec<String>,
}

impl<T: Object> JsonChanged<T> {
    pub fn new(json_map: JsonMap<T>, diff: Vec<String>) -> Self {
        Self {
            json_map,
            fields_diff: diff,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComponentMap {
    map: HashMap<String, ComponentData>,
}

impl From<HashMap<String, ComponentData>> for ComponentMap {
    fn from(value: HashMap<String, ComponentData>) -> Self {
        ComponentMap { map: value }
    }
}

impl From<ComponentMap> for HashMap<String, ComponentData> {
    fn from(value: ComponentMap) -> Self {
        value.map
    }
}

impl ComponentMap {
    pub fn new(map: HashMap<String, ComponentData>) -> Self {
        Self { map }
    }

    pub fn object(&self) -> serde_json::Value {
        let mut value = json! { {} };
        let map = value.as_object_mut().unwrap();
        for (key, value) in &self.map {
            map.insert(key.clone(), value.value.clone());
        }
        value
    }

    pub fn get_value_str(
        &self,
        field_name: &str,
        value_type: ValueType,
    ) -> Result<Option<String>, serde_json::Error> {
        let value = get_field_to_str(&self.object(), field_name, value_type);
        Ok(value)
    }

    pub fn into_map(self) -> HashMap<String, ComponentData> {
        self.map
    }

    pub fn map(&self) -> &HashMap<String, ComponentData> {
        &self.map
    }
}

#[derive(Debug, Clone)]
pub struct ObjectModel<T: Object> {
    pub public_object_writer: WriteSignal<T>,
    pub public_component_reader: Memo<(JsonMap<T>, ComponentMap)>,
    pub public_object_reader: Memo<T>,
}

impl<T: Object> ObjectModel<T> {
    pub fn signal(&self) -> (Memo<T>, WriteSignal<T>) {
        (self.public_object_reader, self.public_object_writer)
    }

    pub fn new(
        cx: Scope,
        object: T,
        validators: Vec<Box<dyn ValidatorProvider + 'static + Send + Sync>>,
    ) -> Self {
        let default_json_map = JsonMap::new(object.clone());

        let (public_to_validate, public_object_writer) = create_signal(cx, object.clone());

        // Intercept public change and extract the changed fields (diff)
        let diff_json_reader = {
            let default_json_map = default_json_map.clone();
            create_memo(cx, move |previous_json_map: Option<&JsonChanged<T>>| {
                info!("1) inside to validate...");
                let object = public_to_validate();
                let json_new = JsonMap::new(object);
                // Try get previous object values
                let json_old = match previous_json_map {
                    Some(json_changed) => json_changed.json_map.clone(),
                    None => default_json_map.clone().into(),
                };
                // Generate diff with the changes
                let diff = modified_fields_name(json_old, json_new.clone());
                JsonChanged::new(json_new, diff)
            })
        };

        // From the changed fields execute validators
        let diff_validated_reader = {
            let validators = validators.clone();
            create_resource(cx, diff_json_reader, move |json_changed| {
                exec_validators(validators.clone(), json_changed)
            })
        };

        // Intercept validated and wait result
        let public_component_reader = {
            let default_json_map = default_json_map.clone();
            create_memo(cx, move |previous: Option<&(JsonMap<T>, ComponentMap)>| {
                info!("3) wait validators result...");
                let previous = previous.cloned().unwrap_or_else(|| {
                    (
                        default_json_map.clone(),
                        object_to_map_comp(&default_json_map),
                    )
                });

                let component_map = diff_validated_reader.read();

                let (user_json_map, new_json_map, component_map) =
                    match component_map.as_ref().cloned() {
                        Some(component_map) => component_map,
                        None => return previous,
                    };

                let mut full_map = previous.clone().1.into_map();

                // Update value to existing component date state
                let map_j = new_json_map.map();
                for (key, value) in map_j {
                    full_map
                        .entry(key.clone())
                        .and_modify(|cd| cd.value = value.clone())
                        .or_insert(value.clone().into());
                }

                // Update whole component data
                for (field_name, component_data) in component_map.into_map() {
                    full_map.insert(field_name.clone(), component_data.clone());
                }

                let component_map = full_map.into();
                info!("3) wait validators result: `{component_map:?}`");

                info!("user_json_map `{user_json_map:?}`");

                (user_json_map, component_map)
            })
        };

        // Read map component data and transform to object
        let public_object_reader = create_memo(cx, move |previous_object: Option<&T>| {
            info!("4) transform to object...");

            let previous_object = previous_object
                .cloned()
                .unwrap_or_else(|| default_json_map.clone().get());

            let mut object_j = serde_json::to_value(&previous_object).unwrap();

            let map_object = object_j.as_object_mut().unwrap();

            let public_component_reader = public_component_reader();
            let component_map = public_component_reader.1.into_map();

            for (filed_name, component_data) in component_map {
                map_object[&filed_name] = component_data.value;
            }

            info!("4) transform to object: `{object_j:?}`");
            serde_json::from_value(object_j).unwrap()
        });

        Self {
            public_object_writer,
            public_component_reader,
            public_object_reader,
        }
    }
}

fn object_to_map_comp<T: Serialize>(object: &T) -> ComponentMap {
    let value_j = serde_json::to_value(object).unwrap();
    let map = value_j.as_object().unwrap();
    let mut components_data = HashMap::<String, ComponentData>::new();
    for (name, value) in map {
        components_data.insert(name.clone(), value.clone().into());
    }
    ComponentMap::new(components_data)
}

async fn exec_validators<T: Object>(
    validators: Vec<Box<dyn ValidatorProvider + 'static + Send + Sync>>,
    json_changed: JsonChanged<T>,
) -> (JsonMap<T>, JsonMap<T>, ComponentMap) {
    info!("2) exec validators json_changed: `{json_changed:?}`...");
    let mut components_data = HashMap::<String, ComponentData>::new();
    let JsonChanged {
        json_map,
        fields_diff,
    } = json_changed;

    let mut json_new = json_map.clone();

    // Iterate thru changed fields
    for field_diff_name in fields_diff {
        // Get validators from each field name
        let validators = validators_by_field(validators.clone(), &field_diff_name);

        let mut field_value = json_new
            .object()
            .get(&field_diff_name)
            .cloned()
            .unwrap_or(().into());

        if validators.is_empty() {
            components_data.insert(field_diff_name.clone(), field_value.into());
        } else {
            for validator in validators {
                // Create validator request
                let request =
                    validator.create_request(&json_new.clone().into(), &field_diff_name.clone());
                // Execute validation
                // info!("inside create_action");
                let response = exec_validator(validator, request).await.unwrap();
                // If there are result values then update values
                if let Some(output_values) = response.opt_output_values {
                    let merged = output_values.merge_to_j(json_new.clone().into());

                    json_new = JsonMap::new(merged);

                    // Retrieving new value
                    field_value = json_new
                        .object()
                        .get(&field_diff_name)
                        .cloned()
                        .unwrap_or(().into());
                }

                let component_data = ComponentData {
                    value: field_value.clone(),
                    hint: response.hint,
                    valid: Some(response.valid),
                };
                components_data.insert(field_diff_name.clone(), component_data);
            }
        }
    }
    let component_map = ComponentMap::new(components_data);
    info!("2) exec validators component_map: `{component_map:?}` json_new: `{json_new:?}`");
    (json_map, json_new, component_map)
}
