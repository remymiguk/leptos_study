use std::collections::HashMap;

use super::{
    form_object::ComponentData,
    json_map::JsonMap,
    validator::{exec_validator, ValidatorProvider},
};
use crate::states::{form_object::validators_by_field, object::Object};
use leptos::*;
use serde::Serialize;
use voxi_core::objects::value_json::modified_fields_name;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct JsonChanged<T: Object> {
    json_map: JsonMap<T>,
    fields_name: Vec<String>,
}

impl<T: Object> JsonChanged<T> {
    pub fn new(json_map: JsonMap<T>, diff: Vec<String>) -> Self {
        Self {
            json_map,
            fields_name: diff,
        }
    }
}

pub struct ConponentMap {
    map: HashMap<String, ComponentData>,
}

pub struct ObjectModel<T: Object> {
    public_write_signal: WriteSignal<JsonMap<T>>,
    public_read_signal: Memo<HashMap<String, ComponentData>>,
}

impl<T: Object> ObjectModel<T> {
    pub fn new(
        cx: Scope,
        object: T,
        validators: Vec<Box<dyn ValidatorProvider + 'static + Send + Sync>>,
    ) -> Self {
        let default_json_map = JsonMap::new(object);

        let (public_to_validate, public_write_signal) = create_signal(cx, default_json_map.clone());

        // Intercept public change and extract the changed fields (diff)
        let changed_json_reader = {
            let default_json_map = default_json_map.clone();
            create_memo(cx, move |previous_json_map: Option<&JsonChanged<T>>| {
                let json_new = public_to_validate();
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
        let validated_reader = {
            let validators = validators.clone();
            create_resource(cx, changed_json_reader, move |json_changed| {
                exec_validators(validators.clone(), json_changed)
            })
        };

        // Intercept validated and wait result
        let public_read_signal = create_memo(
            cx,
            move |prev_map: Option<&HashMap<String, ComponentData>>| {
                let json_map_validated = match validated_reader.read().as_ref() {
                    Some(json_map_validated) => json_map_validated.clone(),
                    None => match prev_map {
                        Some(previous_json_map) => return previous_json_map.clone(),
                        None => return object_to_map_comp(&default_json_map),
                    },
                };
                json_map_validated
            },
        );

        Self {
            public_write_signal,
            public_read_signal,
        }
    }
}

fn object_to_map_comp<T: Serialize>(object: &T) -> HashMap<String, ComponentData> {
    let value_j = serde_json::to_value(object).unwrap();
    let map = value_j.as_object().unwrap();
    let mut components_data = HashMap::<String, ComponentData>::new();
    for (name, value) in map {
        components_data.insert(name.clone(), value.clone().into());
    }
    components_data
}

async fn exec_validators<T: Object>(
    validators: Vec<Box<dyn ValidatorProvider + 'static + Send + Sync>>,
    json_changed: JsonChanged<T>,
) -> HashMap<String, ComponentData> {
    let mut components_data = HashMap::<String, ComponentData>::new();
    let JsonChanged {
        json_map: mut json_new,
        fields_name,
    } = json_changed;
    // Iterate thru changed fields
    for field_name in fields_name {
        // Get validators from each field name
        let validators = validators_by_field(validators.clone(), &field_name);

        let field_value = json_new
            .object()
            .get(&field_name)
            .cloned()
            .unwrap_or(().into());

        if validators.is_empty() {
            components_data.insert(field_name.clone(), field_value.into());
        } else {
            for validator in validators {
                // Create validator request
                let request =
                    validator.create_request(&json_new.clone().into(), &field_name.clone());
                // Execute validation
                // info!("inside create_action");
                let response = exec_validator(validator, request).await.unwrap();
                // If there are result values then update values
                if let Some(subset_values) = response.opt_subset_values {
                    let merged = subset_values.merge_to_j(json_new.clone().into());
                    json_new = JsonMap::new(merged);
                }

                let component_data = ComponentData {
                    value: field_value.clone(),
                    hint: response.hint,
                    valid: response.valid,
                };
                components_data.insert(field_name.clone(), component_data);
            }
        }
    }
    components_data
}
