use crate::states::input_type::InputType;
use derive_builder::Builder;
use rust_decimal::Decimal;

use super::input_mode::InputMode;

/// https://www.w3schools.com/html/html_form_attributes.asp
#[derive(Builder, Default, Debug, Clone, PartialEq, Eq)]
#[builder(setter(strip_option))]
#[builder(default)]
pub struct InputAttributes {
    pub input_type: InputType,
    pub placeholder: Option<String>,
    pub inputmode: Option<InputMode>,
    pub readonly: Option<bool>,
    pub disabled: Option<bool>,
    // The size attribute works with the following input types: text, search, tel, url, email, and password.
    pub size: Option<usize>,
    pub maxlength: Option<usize>,
    // The multiple attribute works with the following input types: email, and file.
    pub multiple: Option<bool>,
    pub min: Option<Decimal>,
    pub max: Option<Decimal>,
    pub pattern: Option<String>,
    pub required: Option<bool>,
    pub autofocus: Option<bool>,
    // The input height and width attributes specify the height and width of an <input type="image"> element
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub datalist: Option<Vec<String>>,
    pub autocomplete: Option<String>,
    pub step: Option<Decimal>,
}

impl InputAttributes {
    pub fn new(i_type: InputType) -> Self {
        Self {
            input_type: i_type,
            ..Default::default()
        }
    }

    pub fn builder() -> InputAttributesBuilder {
        InputAttributesBuilder::default()
    }

    pub fn min(mut self, value: Decimal) -> Self {
        self.min = Some(value);
        self
    }

    pub fn max(mut self, value: Decimal) -> Self {
        self.max = Some(value);
        self
    }

    pub fn step(mut self, value: Decimal) -> Self {
        self.step = Some(value);
        self
    }
}
