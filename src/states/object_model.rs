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

pub struct ComponentMap {
    map: HashMap<String, ComponentData>,
}

pub struct ObjectModel<T: Object> {
    // FIXME: use T instead JsonMap ????
    public_object_writer: WriteSignal<JsonMap<T>>,
    public_component_reader: Memo<HashMap<String, ComponentData>>,
    public_object_reader: Memo<T>,
}

impl<T: Object> ObjectModel<T> {
    pub fn new(
        cx: Scope,
        object: T,
        validators: Vec<Box<dyn ValidatorProvider + 'static + Send + Sync>>,
    ) -> Self {
        let default_json_map = JsonMap::new(object);

        let (public_to_validate, public_object_writer) =
            create_signal(cx, default_json_map.clone());

        // Intercept public change and extract the changed fields (diff)
        let diff_json_reader = {
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
        let diff_validated_reader = {
            let validators = validators.clone();
            create_resource(cx, diff_json_reader, move |json_changed| {
                exec_validators(validators.clone(), json_changed)
            })
        };

        // Intercept validated and wait result
        let public_component_reader = {
            let default_json_map = default_json_map.clone();
            create_memo(
                cx,
                move |full_map: Option<&HashMap<String, ComponentData>>| {
                    let mut full_map = full_map
                        .cloned()
                        .unwrap_or_else(|| object_to_map_comp(&default_json_map));

                    let diff_validated_reader = diff_validated_reader.read();

                    let json_map_validated = match diff_validated_reader.as_ref() {
                        Some(json_map_validated) => json_map_validated,
                        None => return full_map,
                    };

                    for (field_name, component_data) in json_map_validated {
                        full_map.insert(field_name.clone(), component_data.clone());
                    }

                    full_map
                },
            )
        };

        // Read map component data and transform to object
        let public_object_reader = create_memo(cx, move |previous_object: Option<&T>| {
            let previous_object = previous_object
                .cloned()
                .unwrap_or_else(|| default_json_map.clone().get());
            let mut object_j = serde_json::to_value(&previous_object).unwrap();
            let map_object = object_j.as_object_mut().unwrap();
            for (filed_name, component_data) in public_component_reader() {
                map_object[&filed_name] = component_data.value;
            }
            previous_object
        });

        Self {
            public_object_writer,
            public_component_reader,
            public_object_reader,
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
        fields_diff,
    } = json_changed;
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
                if let Some(subset_values) = response.opt_subset_values {
                    let merged = subset_values.merge_to_j(json_new.clone().into());

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
                    valid: response.valid,
                };
                components_data.insert(field_diff_name.clone(), component_data);
            }
        }
    }
    components_data
}
