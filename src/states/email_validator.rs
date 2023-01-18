use super::validator::{ValidatorProvider, ValidatorRequest, ValidatorResponse};
use async_trait::async_trait;
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};
use voxi_core::{FieldNameType, IntoFieldNameType, ValueType};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ValidatorEmail {
    field_name_type: FieldNameType,
    input_fields: Vec<FieldNameType>,
}

impl ValidatorEmail {
    pub fn new() -> Self {
        Self {
            field_name_type: (ValueType::String, "email").into_field_name_type(),
            input_fields: vec![],
        }
    }
}

#[async_trait]
impl ValidatorProvider for ValidatorEmail {
    fn trigger_field_name(&self) -> &FieldNameType {
        &self.field_name_type
    }

    fn input_fields_name(&self) -> &Vec<FieldNameType> {
        &self.input_fields
    }

    async fn validate(&self, request: ValidatorRequest) -> Result<ValidatorResponse, String> {
        #[derive(Serialize, Deserialize)]
        struct PasswordEmail {
            email: Option<String>,
        }

        let input: PasswordEmail = request.input_values.object();

        let is_valid = EmailAddress::is_valid(&input.email.unwrap_or_default());
        let hint = if is_valid {
            "Valid e-mail"
        } else {
            "Invalid e-mail"
        };

        let response = ValidatorResponse {
            hint: Some(hint.into()),
            is_valid,
            opt_output_values: None,
        };

        Ok(response)
    }
}
