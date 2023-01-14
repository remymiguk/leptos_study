use super::form_json::FormJson;
use leptos::*;
use log::info;
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;
use voxi_core::ValueType;
use web_sys::Event;

#[derive(Clone)]
pub struct FormObject<T: Serialize + DeserializeOwned + Clone + 'static> {
    _phantom: PhantomData<T>,
    cx: Scope,
    read_signal: ReadSignal<FormJson<T>>,
    write_signal: WriteSignal<FormJson<T>>,
}

impl<T: Serialize + DeserializeOwned + Clone + 'static> FormObject<T> {
    pub fn new(cx: Scope, object: T) -> Self {
        let (read_signal, write_signal) = create_signal(cx, FormJson::try_from(object).unwrap());
        Self {
            cx,
            _phantom: Default::default(),
            read_signal,
            write_signal,
        }
    }

    pub fn input_bind(&self, field_name: &str) -> impl IntoView {
        let field_name = field_name.to_string();
        let content = self.memo_content_map(self.cx, field_name.clone(), ValueType::String);
        let on_input = self.event_to_map(field_name, ValueType::String);

        let cx = self.cx;
        view! {
            cx,
            <input class="input is-primary" type="text" placeholder="Primary input"
                on:input=on_input
                prop:value=content
            />
        }
    }

    fn memo_content_map(
        &self,
        cx: Scope,
        field_name: String,
        value_type: ValueType,
    ) -> Memo<String> {
        let read_signal = self.read_signal;
        create_memo(cx, move |_| {
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

    pub fn signal(&self) -> (ReadSignal<FormJson<T>>, WriteSignal<FormJson<T>>) {
        (self.read_signal, self.write_signal)
    }

    pub fn read_signal(&self) -> ReadSignal<FormJson<T>> {
        self.read_signal
    }

    pub fn write_signal(&self) -> WriteSignal<FormJson<T>> {
        self.write_signal
    }

    pub fn get(&self) -> T {
        let read_signal = self.read_signal();
        read_signal().get()
    }

    pub fn set(&self, object: T) {
        let write_signal = self.write_signal();
        let json = FormJson::try_from(object).unwrap();
        write_signal.set(json);
    }
}
