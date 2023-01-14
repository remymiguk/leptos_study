use super::{
    json_map::JsonMap,
    validator::{exec_validator, ValidatorProvider},
};
use crate::states::input_mode::InputMode;
use leptos::*;
use log::info;
use serde::{de::DeserializeOwned, Serialize};
use voxi_core::{
    objects::{sub_set_values::subset_values_to_object_j, value_json::get_field_to_str},
    ValueType,
};
use web_sys::Event;

#[derive(Clone)]
pub struct FormObject<T: Serialize + DeserializeOwned + Clone + 'static> {
    objeto_read_signal: ReadSignal<JsonMap<T>>,
    objeto_write_signal: WriteSignal<JsonMap<T>>,

    valid_read_signal: ReadSignal<bool>,
    valid_write_signal: WriteSignal<bool>,

    hint_read_signal: ReadSignal<Option<String>>,
    hint_write_signal: WriteSignal<Option<String>>,

    validator: Option<Box<dyn ValidatorProvider>>,
}

impl<T: Serialize + DeserializeOwned + Clone + 'static> FormObject<T> {
    pub fn new(cx: Scope, object: T) -> Self {
        let (objeto_read_signal, objeto_write_signal) =
            create_signal(cx, JsonMap::try_from(object).unwrap());
        let (valid_read_signal, valid_write_signal) = create_signal(cx, true);
        let (hint_read_signal, hint_write_signal) = create_signal(cx, Option::<String>::None);
        Self {
            objeto_read_signal,
            objeto_write_signal,
            validator: None,
            valid_read_signal,
            valid_write_signal,
            hint_read_signal,
            hint_write_signal,
        }
    }

    pub fn with_validator(mut self, validator: Box<dyn ValidatorProvider + 'static>) -> Self {
        self.validator = Some(validator);
        self
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
        let on_input = self.event_to_map(field_name, ValueType::String);

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

        let validator_action_opt = self
            .validator
            .as_ref()
            .filter(|validator| validator.field_name().name.name() == field_name)
            .cloned();

        let hint_write_signal = self.hint_write_signal;
        let valid_write_signal = self.valid_write_signal;

        let object_write_signal = self.objeto_write_signal;

        let read_signal = self.objeto_read_signal;

        create_memo(cx, move |_| {
            let json_map = read_signal();
            let old_json = json_map.object().clone();

            let validator_action_opt = validator_action_opt.as_ref().map(|validator| {
                let validator = validator.clone();
                let field_name = field_name.clone();
                create_action(cx, move |object_j: &serde_json::Value| {
                    let request = validator.create_request(object_j, &field_name.clone());
                    info!("inside create_action");
                    exec_validator(validator.clone(), request)
                })
            });

            let current_json = match validator_action_opt {
                Some(validator_action) => {
                    info!("validator_action.dispatch");

                    validator_action.dispatch(old_json.clone());
                    let action_value = validator_action.value();

                    info!("action_value: {action_value:?}");

                    match action_value() {
                        Some(Ok(response)) => {
                            info!("#### response: {response:?}");

                            hint_write_signal.set(response.hint);
                            valid_write_signal.set(response.valid);

                            if let Some(sub_set_values) = response.opt_subset_values {
                                let new_json =
                                    subset_values_to_object_j(&sub_set_values, old_json.clone());
                                if new_json != old_json {
                                    info!("diff new_json {new_json:?} old_json {old_json:?}");

                                    let object: T =
                                        serde_json::from_value(new_json.clone()).unwrap();
                                    object_write_signal.set(object.into())
                                }
                                new_json
                            } else {
                                old_json
                            }
                        }
                        _ => {
                            info!("other...");
                            old_json
                        }
                    }
                }
                None => old_json,
            };

            let value_s =
                get_field_to_str(&current_json, &field_name, value_type).unwrap_or_default();
            info!("**** inside memo {value_s} ...");
            value_s
        })
    }

    fn event_to_map(&self, field_name: String, value_type: ValueType) -> impl Fn(Event) {
        let read_signal = self.objeto_read_signal;
        let write_signal = self.objeto_write_signal;
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
        (self.objeto_read_signal, self.objeto_write_signal)
    }

    pub fn read_signal(&self) -> ReadSignal<JsonMap<T>> {
        self.objeto_read_signal
    }

    // pub fn write_signal(&self) -> WriteSignal<JsonMap<T>> {
    //     self.objeto_write_signal
    // }

    pub fn get(&self) -> T {
        let read_signal = self.read_signal();
        read_signal().get()
    }

    pub fn set(&self, object: T) {
        let write_signal = self.objeto_write_signal;
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
    T: Serialize + DeserializeOwned + Clone + 'static,
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
