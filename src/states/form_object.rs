use super::{
    json_map::JsonMap,
    validator::{exec_validator, ValidatorProvider, ValidatorRequest, ValidatorResponse},
};
use leptos::*;
use log::info;
use serde::{de::DeserializeOwned, Serialize};
use voxi_core::{objects::sub_set_values::subset_values_to_object_j, ValueType};
use web_sys::Event;

#[derive(Clone)]
pub struct FormObject<T: Serialize + DeserializeOwned + Clone + 'static> {
    read_signal: ReadSignal<JsonMap<T>>,
    write_signal: WriteSignal<JsonMap<T>>,
    validator: Option<Box<dyn ValidatorProvider>>,
}

impl<T: Serialize + DeserializeOwned + Clone + 'static> FormObject<T> {
    pub fn new(cx: Scope, object: T) -> Self {
        let (read_signal, write_signal) = create_signal(cx, JsonMap::try_from(object).unwrap());
        Self {
            read_signal,
            write_signal,
            validator: None,
        }
    }

    pub fn input_bind_text(&self, cx: Scope, field_name: &str) -> impl IntoView {
        let field_name = field_name.to_string();
        let content = self.memo_content_map(cx, field_name.clone(), ValueType::String);
        let on_input = self.event_to_map(field_name, ValueType::String);
        let classes = "input is-primary";
        view! {
            cx,
            <input class={classes} type="text" placeholder="Primary input"
                on:input=on_input
                prop:value=content
            />
        }
    }

    // TODO: should returns T
    fn memo_content_map(
        &self,
        cx: Scope,
        field_name: String,
        value_type: ValueType,
    ) -> Memo<String> {
        let read_signal = self.read_signal();
        let write_signal = self.write_signal();

        let validator_action_opt = self.validator.as_ref().map(|validator| {
            let validator = validator.clone();

            create_action(cx, move |field_name: &String| {
                let json_map = read_signal.get();
                let object_j = json_map.object();
                let request = validator.create_request(object_j, &field_name.clone());
                exec_validator(validator.clone(), request)
            })
        });

        let read_signal = self.read_signal;
        create_memo(cx, move |_| {
            if let Some(validator_action) = validator_action_opt {
                validator_action.dispatch(field_name.to_string());
                let action_value = validator_action.value();
                if let Some(validator_result) = action_value() {
                    info!("{validator_result:?}");
                    match validator_result {
                        Ok(response) => {
                            if let Some(sub_set_values) = response.opt_subset_values {
                                let json_map = read_signal.get();
                                let object_j = json_map.object();
                                let new_j =
                                    subset_values_to_object_j(&sub_set_values, object_j.clone());
                                let object: T = serde_json::from_value(new_j).unwrap();
                                write_signal.set(object.into())
                            }
                        }
                        Err(_) => todo!(),
                    }
                }
            }
            read_signal()
                .get_value_str(&field_name, value_type)
                .unwrap()
                .unwrap_or_default()
        })
    }

    fn event_to_map(&self, field_name: String, value_type: ValueType) -> impl Fn(Event) {
        let read_signal = self.read_signal;
        let write_signal = self.write_signal;
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
        (self.read_signal, self.write_signal)
    }

    pub fn read_signal(&self) -> ReadSignal<JsonMap<T>> {
        self.read_signal
    }

    pub fn write_signal(&self) -> WriteSignal<JsonMap<T>> {
        self.write_signal
    }

    pub fn get(&self) -> T {
        let read_signal = self.read_signal();
        read_signal().get()
    }

    pub fn set(&self, object: T) {
        let write_signal = self.write_signal();
        let json = JsonMap::try_from(object).unwrap();
        write_signal.set(json);
    }
}

#[component]
pub fn InputBind<T, 'a>(cx: Scope, fo: &'a FormObject<T>, field_name: &'a str) -> impl IntoView
where
    T: Serialize + DeserializeOwned + Clone + 'static,
{
    view! {
        cx,
        <>
            { fo.input_bind_text(cx, field_name) }
        </>
    }
}
