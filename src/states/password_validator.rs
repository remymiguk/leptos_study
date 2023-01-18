use super::validator::{ValidatorProvider, ValidatorRequest, ValidatorResponse};
use async_trait::async_trait;
use log::info;
use serde::{Deserialize, Serialize};
use voxi_core::{
    objects::sub_set_values::SubsetValues, FieldNameType, IntoFieldNameType, ValueType,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ValidatorPassword {
    field_name_type: FieldNameType,
    input_fields: Vec<FieldNameType>,
}

impl ValidatorPassword {
    pub fn new() -> Self {
        Self {
            field_name_type: (ValueType::String, "password").into_field_name_type(),
            input_fields: vec![(ValueType::String, "email").into_field_name_type()],
        }
    }
}

#[async_trait]
impl ValidatorProvider for ValidatorPassword {
    fn trigger_field_name(&self) -> &FieldNameType {
        &self.field_name_type
    }

    fn input_fields_name(&self) -> &Vec<FieldNameType> {
        &self.input_fields
    }

    async fn validate(&self, request: ValidatorRequest) -> Result<ValidatorResponse, String> {
        #[derive(Serialize, Deserialize)]
        struct PasswordEmail {
            password: Option<String>,
        }

        let input: PasswordEmail = request.input_values.object();
        let password = input.password.unwrap_or_default();

        let is_valid = password.len() > 8;
        let hint = if !is_valid {
            Some("Password must be at least 8 characters long".to_string())
        } else {
            None
        };

        let response = ValidatorResponse {
            hint,
            is_valid,
            opt_output_values: None,
        };

        Ok(response)
    }
}
