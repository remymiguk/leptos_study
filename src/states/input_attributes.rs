use crate::states::input_type::InputType;
use derive_builder::Builder;
use rust_decimal::Decimal;

use super::input_mode::InputMode;

/// https://www.w3schools.com/html/html_form_attributes.asp
#[derive(Builder, Default, Debug, Clone, PartialEq, Eq)]
#[builder(setter(strip_option))]
#[builder(default)]
pub struct InputAttributes {
    pub name: String,
    pub i_type: InputType,
    pub placeholder: Option<String>,
    pub input_mode: Option<InputMode>,
    pub readonly: bool,
    pub disabled: bool,
    // The size attribute works with the following input types: text, search, tel, url, email, and password.
    pub size: Option<u16>,
    pub maxlength: Option<u16>,
    // The multiple attribute works with the following input types: email, and file.
    pub multiple: bool,
    pub min: Option<Decimal>,
    pub max: Option<Decimal>,
    pub pattern: Option<String>,
    pub required: bool,
    pub autofocus: bool,
    // The input height and width attributes specify the height and width of an <input type="image"> element
    pub width: Option<u16>,
    pub height: Option<u16>,
    pub datalist: Option<Vec<String>>,
    pub autocomplete: Option<bool>,
    pub step: Option<Decimal>,
}

impl InputAttributes {
    pub fn new(name: &str, i_type: InputType) -> Self {
        Self {
            name: name.into(),
            i_type,
            ..Default::default()
        }
    }

    pub fn builder() -> InputAttributesBuilder {
        InputAttributesBuilder::default()
    }
}
