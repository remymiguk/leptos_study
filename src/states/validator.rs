use async_trait::async_trait;
use dyn_clonable::clonable;
use log::info;
use serde::{Deserialize, Serialize};
use voxi_core::{
    objects::sub_set_values::{object_j_to_subset_values, SubsetValues},
    FieldNameType, IntoFieldName, IntoFieldNameType, IntoValueType,
};

#[clonable]
#[async_trait]
pub trait ValidatorProvider: Clone + std::fmt::Debug {
    async fn validate(&self, request: ValidatorRequest) -> Result<ValidatorResponse, String>;

    fn trigger_field_name(&self) -> &FieldNameType;

    fn input_fields_name(&self) -> &Vec<FieldNameType>;

    fn all_input_fields(&self) -> Vec<&FieldNameType> {
        let mut fields = vec![self.trigger_field_name()];
        for f in self.input_fields_name() {
            fields.push(f);
        }
        fields
    }

    fn create_request(
        &self,
        object_j: &serde_json::Value,
        trigger_field_name: &str,
    ) -> ValidatorRequest {
        let input_values = object_j_to_subset_values(object_j, self.all_input_fields()).unwrap();
        ValidatorRequest {
            input_values,
            trigger_field_name: trigger_field_name.to_string(),
        }
    }
}

pub async fn exec_validator(
    validator: Box<dyn ValidatorProvider>,
    request: ValidatorRequest,
) -> Result<ValidatorResponse, String> {
    validator.validate(request).await
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ValidatorPassword {
    field_name_type: FieldNameType,
    input_fields: Vec<FieldNameType>,
}

impl ValidatorPassword {
    pub fn new(v_type: impl IntoValueType, field_name: impl IntoFieldName) -> Self {
        let field_name_type = (v_type, field_name).into_field_name_type();
        Self {
            field_name_type,
            input_fields: Vec::default(),
        }
    }

    pub fn add_input(
        mut self,
        v_type: impl IntoValueType,
        field_name: impl IntoFieldName,
    ) -> ValidatorPassword {
        let field_name_type = (v_type, field_name).into_field_name_type();
        self.input_fields.push(field_name_type);
        self
    }
}

impl std::fmt::Display for ValidatorPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.field_name_type)
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
            email: Option<String>,
        }

        let mut input: PasswordEmail = request.input_values.object();

        let mut password = input.password.unwrap_or_default();
        password.truncate(10);

        input.password = Some(password.clone());

        input.email = Some(format!("{password}@gmail.com"));

        let subset_values = SubsetValues::from_object(&input, self.all_input_fields()).unwrap();

        let response = ValidatorResponse {
            hint: Some(format!("hint: {:?}", input.password)),
            valid: input.password.unwrap_or_default().len() % 2 == 0,
            opt_output_values: Some(subset_values),
        };

        info!("inside validator response: `{response:?}`");

        Ok(response)
    }
}

#[derive(Debug, Clone)]
pub struct ValidatorRequest {
    pub trigger_field_name: String,
    pub input_values: SubsetValues,
}

impl ValidatorRequest {
    pub fn new(trigger_field_name: &str, subset_values: SubsetValues) -> Self {
        Self {
            trigger_field_name: trigger_field_name.to_string(),
            input_values: subset_values,
        }
    }
}

impl PartialEq for ValidatorRequest {
    fn eq(&self, other: &Self) -> bool {
        self.input_values == other.input_values
            && self.trigger_field_name == other.trigger_field_name
    }
}

#[derive(Debug, Clone)]
pub struct ValidatorRequestCommand {
    pub validator_provider: Box<dyn ValidatorProvider + 'static + Send + Sync>,
    pub request: ValidatorRequest,
}

impl PartialEq for ValidatorRequestCommand {
    fn eq(&self, other: &Self) -> bool {
        self.request == other.request
    }
}

impl ValidatorRequestCommand {
    pub fn new(
        validator_provider: Box<dyn ValidatorProvider + 'static + Send + Sync>,
        request: ValidatorRequest,
    ) -> Self {
        Self {
            request,
            validator_provider,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatorResponse {
    pub hint: Option<String>,
    pub valid: bool,
    pub opt_output_values: Option<SubsetValues>,
}
