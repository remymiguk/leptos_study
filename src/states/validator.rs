use async_trait::async_trait;
use dyn_clonable::clonable;
use voxi_core::{
    objects::sub_set_values::{object_j_to_subset_values, SubsetValues},
    FieldNameType,
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

pub struct Validators {
    validators: Vec<Box<dyn ValidatorProvider + 'static + Send + Sync>>,
}

impl Validators {
    pub fn new() -> Self {
        Self { validators: vec![] }
    }

    pub fn add(mut self, validator: impl ValidatorProvider + 'static + Send + Sync) -> Self {
        self.validators.push(Box::new(validator));
        self
    }

    pub fn into_vec(self) -> Vec<Box<dyn ValidatorProvider + 'static + Send + Sync>> {
        self.into()
    }
}

impl From<Vec<Box<dyn ValidatorProvider + 'static + Send + Sync>>> for Validators {
    fn from(value: Vec<Box<dyn ValidatorProvider + 'static + Send + Sync>>) -> Self {
        Self { validators: value }
    }
}

impl From<Validators> for Vec<Box<dyn ValidatorProvider + 'static + Send + Sync>> {
    fn from(value: Validators) -> Self {
        value.validators
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
    pub is_valid: bool,
    pub opt_output_values: Option<SubsetValues>,
}
