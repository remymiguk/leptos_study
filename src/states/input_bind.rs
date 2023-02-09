use crate::states::input_bind_type::InputBindType;
use crate::states::input_value_type::{InputValueType, IntoInputValueType};
use crate::states::object::Object;
use crate::states::{form_object::FormObject, input_mode::InputMode};
use leptos::*;
use log::info;
use rust_decimal::Decimal;

// pub struct InputClosure {
//     callback: Box<dyn Fn(String) -> String>,
// }

// impl InputValidator for InputClosure {
//     fn value(&self, input: String) -> String {
//         (self.callback)(input)
//     }
// }

#[component]
pub fn InputBind<T, 'a>(
    cx: Scope,
    fo: &'a FormObject<T>,
    #[prop(into)] input_type: String,
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
    #[prop(optional)] min: Option<Decimal>,
    #[prop(optional)] max: Option<Decimal>,
    #[prop(optional)] pattern: Option<String>,
    #[prop(optional)] width: Option<usize>,
    #[prop(optional)] height: Option<usize>,
    #[prop(optional)] step: Option<Decimal>,
    #[prop(optional)] autocomplete: Option<String>,
) -> impl IntoView
where
    T: Object,
{
    let ibt: InputBindType = (&input_type[..]).try_into().unwrap();

    let InputValueType(mut input_attributes, value_type) = ibt.into_input_value_type();
    input_attributes.readonly = input_attributes.readonly.or(readonly);
    input_attributes.disabled = input_attributes.disabled.or(disabled);
    input_attributes.required = input_attributes.required.or(required);
    input_attributes.placeholder = input_attributes.placeholder.or(placeholder);
    input_attributes.inputmode = input_attributes.inputmode.or(inputmode);
    input_attributes.autofocus = input_attributes.autofocus.or(autofocus);
    input_attributes.multiple = input_attributes.multiple.or(multiple);
    input_attributes.size = input_attributes.size.or(size);
    input_attributes.maxlength = input_attributes.maxlength.or(maxlength);
    input_attributes.min = input_attributes.min.or(min);
    input_attributes.max = input_attributes.max.or(max);
    input_attributes.pattern = input_attributes.pattern.or(pattern);
    input_attributes.width = input_attributes.width.or(width);
    input_attributes.height = input_attributes.height.or(height);
    input_attributes.step = input_attributes.step.or(step);
    input_attributes.autocomplete = input_attributes.autocomplete.or(autocomplete);
    let validator = input_attributes.validator.clone();

    let input_ref = NodeRef::<HtmlElement<Input>>::new(cx);

    let content_signal = fo.memo_content(cx, field_name.clone(), value_type);
    let content_s = move || {
        let content_signal = content_signal();
        info!("content_signal: `{content_signal:?}`");
        let value_s = content_signal.0;
        input_ref
            .get()
            .expect("input element to exist")
            .set_value(&value_s);
        value_s
    };

    let is_valid_signal = fo.memo_valid(cx, field_name.clone());
    let hint_signal = fo.memo_hint(cx, field_name.clone());

    let on_input = fo.on_input_to_map(field_name, value_type, input_ref, validator);

    let is_success_read = create_memo(cx, move |_| {
        if is_valid_signal() {
            ("is-success", "fa-check")
        } else {
            ("is-danger", "fa-exclamation-triangle")
        }
    });

    let hint_bottom = create_memo(cx, move |_| {
        hint_signal().map(|hint| {
            let is_success = is_success_read().0;
            view! {
                cx,
                <p class={format!("help {is_success}")}>{hint}</p>
            }
            .into_view(cx)
        })
    });

    let inputmode = input_attributes.inputmode.map(|im| im.to_string());

    let classes_div = move || {
        format!(
            "control has-icons-left has-icons-right {}",
            is_success_read().0
        )
    };
    let classes_input = move || format!("input {}", is_success_read().1);

    let min = input_attributes.min.map(|n| n.to_string());
    let max = input_attributes.max.map(|n| n.to_string());
    let step = input_attributes.step.map(|n| n.to_string());

    let input_type = input_attributes.input_type.to_string();

    view! {
        cx,
        <div class="field">
            <label class="label">{literal}</label>
            <div class={classes_div}>
                <input class={classes_input} type={input_type}
                    _ref={input_ref}
                    {readonly} {disabled} {required} placeholder={input_attributes.placeholder}
                    inputmode={inputmode} {autofocus} {multiple} size={input_attributes.size} maxlength={input_attributes.maxlength}
                    min={min} max={max} pattern={input_attributes.pattern} width={input_attributes.width} height={input_attributes.height} step={step}
                    autocomplete={input_attributes.autocomplete}
                    on:input=on_input
                    prop:value=content_s
                />
            </div>
            { hint_bottom }
        </div>
    }
}
