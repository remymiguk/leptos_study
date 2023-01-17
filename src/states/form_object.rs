use super::{json_map::JsonMap, validator::ValidatorProvider};
use crate::states::input_mode::InputMode;
use crate::states::object::Object;
use leptos::*;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use voxi_core::{objects::value_json::get_field_to_str, ValueType};
use web_sys::Event;

#[derive(Clone)]
pub struct ComponentAtts {
    literal: String,
    field_name: String,
    v_type: ValueType,
    data: ComponentData,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ComponentData {
    pub value: serde_json::Value,
    pub hint: Option<String>,
    pub valid: bool,
}

impl ComponentData {
    pub fn new(value: serde_json::Value) -> Self {
        Self {
            value,
            valid: true,
            ..Default::default()
        }
    }
}

impl From<serde_json::Value> for ComponentData {
    fn from(value: serde_json::Value) -> Self {
        Self::new(value)
    }
}

#[derive(Clone)]
pub struct FormObject<T: Object> {
    object_read_signal: ReadSignal<JsonMap<T>>,
    object_write_signal: WriteSignal<JsonMap<T>>,

    component_read_signal: HashMap<String, ReadSignal<ComponentData>>,
    component_write_signal: HashMap<String, WriteSignal<ComponentData>>,

    validators: Vec<Box<dyn ValidatorProvider + 'static + Send + Sync>>,

    valid_read_signal: ReadSignal<bool>,
    hint_read_signal: ReadSignal<Option<String>>,
}

pub fn validators_by_field(
    mut validators: Vec<Box<dyn ValidatorProvider + 'static + Send + Sync>>,
    field_name: &str,
) -> Vec<Box<dyn ValidatorProvider + 'static + Send + Sync>> {
    validators.retain(|v| v.field_name().name.name() == field_name);
    validators
}

impl<T: Object> FormObject<T> {
    pub fn new(
        cx: Scope,
        object: T,
        validators: Vec<Box<dyn ValidatorProvider + 'static + Send + Sync>>,
    ) -> Self {
        let (object_read_signal, object_write_signal) =
            create_signal(cx, JsonMap::try_from(object.clone()).unwrap());

        let object_j = serde_json::to_value(&object).unwrap();

        let mut component_read_signal = HashMap::new();

        let mut component_write_signal = HashMap::new();

        for (field_name, value) in object_j.as_object().unwrap() {
            let (read_signal, write_signal) = create_signal(cx, ComponentData::new(value.clone()));
            component_read_signal.insert(field_name.to_string(), read_signal);

            let vs = validators_by_field(validators.clone(), field_name);

            // maybe use create_resource ???

            // 1) create action to execute validator
            //// create_action(cx, || {});

            // 2) create memo to read from component and write to object
            // 3) create memo to read from object and write to component

            component_write_signal.insert(field_name.to_string(), write_signal);
        }

        let (valid_read_signal, valid_write_signal) = create_signal(cx, true);
        let (hint_read_signal, hint_write_signal) = create_signal(cx, Option::<String>::None);

        Self {
            object_read_signal,
            object_write_signal,
            validators,
            component_read_signal,
            component_write_signal,
            valid_read_signal,
            hint_read_signal,
        }
    }

    #[allow(unused_variables, clippy::too_many_arguments)]
    pub fn input_bind_text(
        &self,
        cx: Scope,
        literal: String,
        field_name: String,
        readonly: Option<bool>,
        disabled: Option<bool>,
        required: Option<bool>,
        placeholder: Option<String>,
        inputmode: Option<InputMode>,
        autofocus: Option<bool>,
        multiple: Option<bool>,
        size: Option<usize>,
        maxlength: Option<usize>,
        min: Option<usize>,
        max: Option<usize>,
        pattern: Option<String>,
        width: Option<usize>,
        height: Option<usize>,
        step: Option<usize>,
    ) -> impl IntoView {
        let content = self.memo_content_map(cx, field_name.clone(), ValueType::String);
        let on_input = self.event_to_map(cx, field_name, ValueType::String);

        let valid_read = self.valid_read_signal;
        let hint_read_signal = self.hint_read_signal;

        let is_success_read = create_memo(cx, move |_| {
            if valid_read() {
                ("is-success", "fa-check")
            } else {
                ("is-danger", "fa-exclamation-triangle")
            }
        });

        let hint_bottom = create_memo(cx, move |_| {
            hint_read_signal().map(|hint| {
                let is_success = is_success_read().0;
                view! {
                    cx,
                    <p class="help {is_success}"> { "hint" }</p>
                }
                .into_view(cx)
            })
        });

        let inputmode = inputmode.map(|im| im.to_string());

        let classes_div = move || {
            format!(
                "control has-icons-left has-icons-right {}",
                is_success_read().0
            )
        };
        let classes_input = move || format!("input {}", is_success_read().1);

        view! {
            cx,
            <div class="field">
                <label class="label">{literal}</label>
                <div class={classes_div}>
                    <input class={classes_input} type="text"
                        {readonly} {disabled} {required} placeholder={placeholder}
                        inputmode={inputmode} {autofocus} {multiple} size={size} maxlength={maxlength}
                        min={min} max={max} pattern={pattern} width={width} height={height} step={step}
                        on:input=on_input
                        prop:value=content
                    />
                </div>
                { hint_bottom }
            </div>
        }
    }

    // TODO: should returns T
    fn memo_content_map(
        &self,
        cx: Scope,
        field_name: String,
        value_type: ValueType,
    ) -> Memo<String> {
        // let object_read_signal = self.read_signal();

        // let validator_action_opt = self
        //     .validators
        //     .as_ref()
        //     .filter(|validator| validator.field_name().name.name() == field_name)
        //     .cloned();

        // let validator_action_opt = validator_action_opt.as_ref().map(|validator| {
        //     let validator = validator.clone();
        //     let field_name = field_name.clone();
        //     create_action(cx, move |object_j: &serde_json::Value| {
        //         let request = validator.create_request(object_j, &field_name.clone());
        //         info!("inside create_action");
        //         exec_validator(validator.clone(), request)
        //     })
        // });

        // let hint_write_signal = self.hint_write_signal;
        // let valid_write_signal = self.valid_write_signal;

        let object_write_signal = self.object_write_signal;

        let read_signal = self.object_read_signal;

        create_memo(cx, move |_| {
            let json_map = read_signal();
            let old_json = json_map.object().clone();

            let current_json = old_json;

            // let current_json = match validator_action_opt {
            //     Some(validator_action) => {
            //         info!("validator_action.dispatch");

            //         validator_action.dispatch(old_json.clone());
            //         let action_value = validator_action.value();

            //         info!("action_value: {action_value:?}");

            //         match action_value() {
            //             Some(Ok(response)) => {
            //                 info!("#### response: {response:?}");

            //                 hint_write_signal.set(response.hint);
            //                 valid_write_signal.set(response.valid);

            //                 if let Some(sub_set_values) = response.opt_subset_values {
            //                     let new_json =
            //                         subset_values_to_object_j(&sub_set_values, old_json.clone());
            //                     if new_json != old_json {
            //                         info!("diff new_json {new_json:?} old_json {old_json:?}");

            //                         let object: T =
            //                             serde_json::from_value(new_json.clone()).unwrap();
            //                         object_write_signal.set(object.into())
            //                     }
            //                     new_json
            //                 } else {
            //                     old_json
            //                 }
            //             }
            //             _ => {
            //                 info!("other...");
            //                 old_json
            //             }
            //         }
            //     }
            //     None => old_json,
            // };

            let value_s =
                get_field_to_str(&current_json, &field_name, value_type).unwrap_or_default();
            info!("**** inside memo {value_s} ...");
            value_s
        })
    }

    fn event_to_map(&self, cx: Scope, field_name: String, value_type: ValueType) -> impl Fn(Event) {
        // let validator_action_opt = self
        //     .validator
        //     .as_ref()
        //     .filter(|validator| validator.field_name().name.name() == field_name)
        //     .cloned();

        // let validator_action_opt = validator_action_opt.as_ref().map(|validator| {
        //     let validator = validator.clone();
        //     let field_name = field_name.clone();
        //     create_action(cx, move |object_j: &serde_json::Value| {
        //         let request = validator.create_request(object_j, &field_name.clone());
        //         info!("inside create_action");
        //         exec_validator(validator.clone(), request)
        //     })
        // });

        let read_signal = self.object_read_signal;
        let write_signal = self.object_write_signal;

        move |e: Event| {
            let value_s = event_target_value(&e);
            let mut form_map = read_signal.get();
            let value_s = if value_s.is_empty() {
                None
            } else {
                Some(value_s)
            };
            form_map
                .set_value_str(&field_name, value_s, value_type)
                .unwrap();
            write_signal.set(form_map);
        }
    }

    pub fn signal(&self) -> (ReadSignal<JsonMap<T>>, WriteSignal<JsonMap<T>>) {
        (self.object_read_signal, self.object_write_signal)
    }

    pub fn read_signal(&self) -> ReadSignal<JsonMap<T>> {
        self.object_read_signal
    }

    pub fn get(&self) -> T {
        let read_signal = self.read_signal();
        read_signal().get()
    }

    pub fn set(&self, object: T) {
        let write_signal = self.object_write_signal;
        let json = JsonMap::try_from(object).unwrap();
        write_signal.set(json);
    }
}

#[component]
pub fn InputBind<T, 'a>(
    cx: Scope,
    fo: &'a FormObject<T>,
    #[prop(into)] literal: String,
    #[prop(into)] field_name: String,
    #[prop(optional)] readonly: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] required: Option<bool>,
    #[prop(into)]
    #[prop(optional)]
    placeholder: Option<String>,
    #[prop(optional)] inputmode: Option<InputMode>,
    #[prop(optional)] autofocus: Option<bool>,
    #[prop(optional)] multiple: Option<bool>,
    #[prop(optional)] size: Option<usize>,
    #[prop(optional)] maxlength: Option<usize>,
    #[prop(optional)] min: Option<usize>,
    #[prop(optional)] max: Option<usize>,
    #[prop(optional)] pattern: Option<String>,
    #[prop(optional)] width: Option<usize>,
    #[prop(optional)] height: Option<usize>,
    #[prop(optional)] step: Option<usize>,
) -> impl IntoView
where
    T: Object,
{
    view! {
        cx,
        <>
            {fo.input_bind_text(cx, literal, field_name, readonly, disabled, required, placeholder, inputmode,
                autofocus, multiple, size, maxlength, min, max, pattern, width, height, step,
            )}
        </>
    }
}
