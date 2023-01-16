use super::{
    json_map::JsonMap,
    validator::{exec_validator, ValidatorProvider},
};
use crate::states::{form_object::validators_by_field, object::Object};
use leptos::*;
use voxi_core::objects::value_json::{diff_json, fields_names_from_object};

// pub async fn validate<T: Object>(json_map: JsonMap<T>) -> JsonMap<T> {
//     json_map.clone()
// }

pub struct ObjectModel<T: Object> {
    public_write_signal: WriteSignal<JsonMap<T>>,
    public_read_signal: Memo<JsonMap<T>>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct JsonChanged<T: Object> {
    json_map: JsonMap<T>,
    diff: serde_json::Value,
}

impl<T: Object> JsonChanged<T> {
    pub fn new(json_map: JsonMap<T>, diff: serde_json::Value) -> Self {
        Self { json_map, diff }
    }
}

impl<T: Object> ObjectModel<T> {
    pub fn new(cx: Scope, object: T, validators: Vec<Box<dyn ValidatorProvider>>) -> Self {
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
                    None => default_json_map.into(),
                };
                // Generate diff with the changes
                let diff = diff_json(json_old, json_new);
                JsonChanged::new(json_new, diff.new)
            })
        };

        // From the changed fields execute validators
        let validated_reader = create_resource(cx, changed_json_reader, |json_changed| async {
            let JsonChanged {
                json_map: json_new,
                diff,
            } = json_changed;
            // Iterate thru changed fields
            for field_name in fields_names_from_object(&diff) {
                // Get validators from each field name
                let validators = validators_by_field(&validators, &field_name);
                for validator in validators {
                    // Create validator request
                    let request = validator.create_request(&json_new.into(), &field_name.clone());
                    // Execute validation
                    // info!("inside create_action");
                    let response = exec_validator(validator, request).await.unwrap();
                    // If there are result values then update values
                    if let Some(subset_values) = response.opt_subset_values {
                        let merged = subset_values.merge_to_j(json_new.into());
                        json_new = JsonMap::new(merged);
                    }
                }
            }
            json_new
        });

        // Intercept validated and wait result
        let public_read_signal = create_memo(cx, move |previous_json_map: Option<&JsonMap<T>>| {
            let json_map_validated = match validated_reader.read().as_ref() {
                Some(json_map_validated) => json_map_validated.clone(),
                None => match previous_json_map {
                    Some(previous_json_map) => return previous_json_map.clone(),
                    None => return default_json_map.clone(),
                },
            };
            json_map_validated
        });

        Self {
            public_write_signal,
            public_read_signal,
        }
    }
}
