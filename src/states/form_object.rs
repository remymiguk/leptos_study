use std::marker::PhantomData;

use super::form_json::FormJson;
use leptos::*;
use serde::{de::DeserializeOwned, Serialize};
use voxi_core::ValueType;
use web_sys::Event;

pub struct FormObject<T: Serialize + DeserializeOwned + 'static> {
    _phantom: PhantomData<T>,
    read_signal: ReadSignal<FormJson>,
    write_signal: WriteSignal<FormJson>,
}

impl<T: Serialize + DeserializeOwned + 'static> FormObject<T> {
    pub fn new(cx: Scope, object: &T) -> Self {
        let (read_signal, write_signal) = create_signal(cx, FormJson::try_from(object).unwrap());

        //let (read_signal, write_signal) = create_json_context(cx, object);
        Self {
            _phantom: Default::default(),
            read_signal,
            write_signal,
        }
    }

    pub fn input_bind(&self, cx: Scope, field_name: &str) -> impl IntoView {
        let field_name = field_name.to_string();
        let content = self.memo_content_map(cx, field_name.clone(), ValueType::String);
        let on_input = self.event_to_map(cx, field_name, ValueType::String);

        view! {
            cx,
            <input class="input is-primary" type="text" placeholder="Primary input"
            on:input=on_input
            value=content/>
        }
    }

    pub fn memo_content_map(
        &self,
        cx: Scope,
        field_name: String,
        value_type: ValueType,
    ) -> Memo<Option<String>> {
        //let read_signal = use_context::<StateGetter<FormJson>>(cx).unwrap().0;
        let read_signal = self.read_signal;
        create_memo(cx, move |_| {
            read_signal()
                .get_value_str(&field_name, value_type)
                .unwrap()
        })
    }

    pub fn event_to_map(
        &self,
        cx: Scope,
        field_name: String,
        value_type: ValueType,
    ) -> impl Fn(Event) {
        let read_signal = self.read_signal;
        let write_signal = self.write_signal;
        //let read_signal = use_context::<StateGetter<FormJson>>(cx).unwrap().0;
        //let write_signal = use_context::<StateSetter<FormJson>>(cx).unwrap().0;
        move |e: Event| {
            let value_s = event_target_value(&e);
            let mut form_map = read_signal();
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

    pub fn read_signal(&self) -> ReadSignal<FormJson> {
        self.read_signal
    }

    pub fn write_signal(&self) -> WriteSignal<FormJson> {
        self.write_signal
    }
}

// pub fn create_json_context<T: Serialize + DeserializeOwned>(
//     cx: Scope,
//     object: &T,
// ) -> (ReadSignal<FormJson>, WriteSignal<FormJson>) {
//     let (read_signal, write_signal) = create_signal(cx, FormJson::try_from(object).unwrap());

//     provide_context(cx, StateGetter(read_signal));
//     provide_context(cx, StateSetter(write_signal));
//     (read_signal, write_signal)
// }

// pub fn memo_content_map(
//     cx: Scope,
//     field_name: String,
//     value_type: ValueType,
// ) -> Memo<Option<String>> {
//     let read_signal = use_context::<StateGetter<FormJson>>(cx).unwrap().0;
//     create_memo(cx, move |_| {
//         read_signal()
//             .get_value_str(&field_name, value_type)
//             .unwrap()
//     })
// }

// pub fn event_to_map(cx: Scope, field_name: String, value_type: ValueType) -> impl Fn(Event) {
//     move |e: Event| {
//         let read_signal = use_context::<StateGetter<FormJson>>(cx).unwrap().0;
//         let value_s = event_target_value(&e);
//         let mut form_map = read_signal();
//         let value_s = if value_s.is_empty() {
//             None
//         } else {
//             Some(value_s)
//         };
//         form_map
//             .set_value_str(&field_name, value_s, value_type)
//             .unwrap();
//         let write_signal = use_context::<StateSetter<FormJson>>(cx).unwrap().0;
//         write_signal.set(form_map);
//     }
// }
