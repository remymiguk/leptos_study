use crate::services::api_reader::reader_data_set_provider;
use crate::services::api_writer::writer_data_set_provider;
use async_trait::async_trait;
use voxi_dataset::{
    domain::{
        domain_data_set_factory::DomainDataSetFactory,
        reader_data_reader_provider::ReaderDataSetProvider, reader_request::ReaderRequest,
    },
    entity::entity_name::{EntityName, IntoEntityName},
    error::DatasetRoxiError,
    writer::writer_request::WriterRequest,
};
use voxi_dataset::domain::domain_data_set_factory::register_domain_data_set_factory;

pub fn initialization() {
    register_domain_data_set_factory(ApiDomainFactory::new("exchange"));
    register_domain_data_set_factory(ApiDomainFactory::new("order"));
    register_domain_data_set_factory(ApiDomainFactory::new("user"));
}

#[derive(Clone)]
pub struct ApiDomainFactory {
    name: EntityName,
    token: Option<String>,
}

impl ApiDomainFactory {
    pub fn new(name: impl IntoEntityName) -> Self {
        Self {
            name: name.into_entity_name(),
            token: None,
        }
    }
}

#[async_trait(?Send)]
impl DomainDataSetFactory for ApiDomainFactory {
    fn name(&self) -> EntityName {
        self.name.clone()
    }

    async fn async_create_writer(
        &self,
        writer_request: WriterRequest,
    ) -> Result<Box<dyn ReaderDataSetProvider + 'static + Send + Sync>, DatasetRoxiError> {
        let provider = writer_data_set_provider(writer_request, self.token.clone())
            .await
            .map_err(|e| DatasetRoxiError::Generic(e.to_string()))?;
        Ok(provider)
    }

    async fn async_create_reader(
        &self,
        reader_request: ReaderRequest,
    ) -> Result<Box<dyn ReaderDataSetProvider + 'static + Send + Sync>, DatasetRoxiError> {
        let provider = reader_data_set_provider(reader_request, self.token.clone())
            .await
            .map_err(|e| DatasetRoxiError::Generic(e.to_string()))?;
        Ok(provider)
    }

    fn create_writer(
        &self,
        _request: WriterRequest,
    ) -> Result<Box<dyn ReaderDataSetProvider + 'static + Send + Sync>, DatasetRoxiError> {
        Err(DatasetRoxiError::Generic("not implemented".into()).into())
    }
    fn create_reader(
        &self,
        _request: ReaderRequest,
    ) -> Result<Box<dyn ReaderDataSetProvider + 'static + Send + Sync>, DatasetRoxiError> {
        Err(DatasetRoxiError::Generic("not implemented".into()).into())
    }
}
