use async_trait::async_trait;
use dyn_clonable::clonable;
use serde::{Deserialize, Serialize};
use voxi_core::{
    objects::sub_set_values::{object_j_to_subset_values, SubsetValues},
    FieldNameType, IntoFieldName, IntoFieldNameType, IntoValueType,
};

#[clonable]
#[async_trait]
pub trait ValidatorProvider: Clone + std::fmt::Debug {
    async fn validate(&self, request: ValidatorRequest) -> Result<ValidatorResponse, String>;

    fn field_name(&self) -> &FieldNameType;

    fn sub_field_names(&self) -> &Vec<FieldNameType>;

    fn fields(&self) -> Vec<&FieldNameType> {
        let mut fields = vec![self.field_name()];
        for f in self.sub_field_names() {
            fields.push(f);
        }
        fields
    }

    fn create_request(&self, object_j: &serde_json::Value, field_name: &str) -> ValidatorRequest {
        let subset_values = object_j_to_subset_values(&object_j, self.fields()).unwrap();
        ValidatorRequest {
            subset_values,
            field_name: field_name.to_string(),
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
pub struct InputValidator {
    field_name_type: FieldNameType,
    sub_fields: Vec<FieldNameType>,
}

impl InputValidator {
    pub fn new(v_type: impl IntoValueType, field_name: impl IntoFieldName) -> Self {
        let field_name_type = (v_type, field_name).into_field_name_type();
        Self {
            field_name_type,
            sub_fields: Vec::default(),
        }
    }

    pub fn add(&mut self, v_type: impl IntoValueType, field_name: impl IntoFieldName) {
        let field_name_type = (v_type, field_name).into_field_name_type();
        self.sub_fields.push(field_name_type);
    }
}

impl std::fmt::Display for InputValidator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.field_name_type)
    }
}

#[async_trait]
impl ValidatorProvider for InputValidator {
    fn field_name(&self) -> &FieldNameType {
        &self.field_name_type
    }

    fn sub_field_names(&self) -> &Vec<FieldNameType> {
        &self.sub_fields
    }

    async fn validate(&self, request: ValidatorRequest) -> Result<ValidatorResponse, String> {
        #[derive(Serialize, Deserialize)]
        struct Fields {
            user_name: Option<String>,
            email: Option<String>,
        }

        let mut fields: Fields = request.subset_values.object();

        let mut user_name = fields.user_name.unwrap_or_default();
        user_name.truncate(10);

        fields.user_name = Some(user_name.clone());

        fields.email = Some(format!("{}@gmail.com", user_name));

        let subset_values = SubsetValues::from_object(&fields, self.fields()).unwrap();

        let response = ValidatorResponse {
            hint: format!("hint: {:?}", fields.user_name),
            valid: fields.user_name.unwrap_or_default().len() % 2 == 0,
            opt_subset_values: Some(subset_values),
        };

        Ok(response)
    }
}

#[derive(Debug, Clone)]
pub struct ValidatorRequest {
    pub field_name: String,
    pub subset_values: SubsetValues,
}

impl ValidatorRequest {
    pub fn new(field_name: &str, subset_values: SubsetValues) -> Self {
        Self {
            field_name: field_name.to_string(),
            subset_values,
        }
    }
}

// impl Hash for ValidatorRequest {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.field_name.hash(state);
//         self.subset_values.hash(state);
//     }
// }

impl PartialEq for ValidatorRequest {
    fn eq(&self, other: &Self) -> bool {
        self.subset_values == other.subset_values && self.field_name == other.field_name
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

// impl Hash for ValidatorRequestCommand {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.request.hash(state);
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatorResponse {
    pub hint: String,
    pub valid: bool,
    pub opt_subset_values: Option<SubsetValues>,
}
