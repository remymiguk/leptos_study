use crate::{app::error::AppError, services::api_request_gloo::APIRequestGloo};
use voxi_core::selections::IntoConditionWhere;
use voxi_dataset::{
    domain::{
        reader_context::ReaderContext, reader_data_reader_provider::ReaderDataSetProvider,
        reader_request::ReaderRequest, reader_response::ReaderResponse,
        response_reader_data_set_provider::ResponseReaderDataSetProvider,
    },
    entity::entity_name::IntoEntityName,
};

pub async fn fetch_reader_response(
    reader_request: ReaderRequest,
    token: Option<String>,
) -> Result<ReaderResponse, AppError> {
    println!("async_get_domain...");
    APIRequestGloo::new()
        .token_opt(token)
        .body(reader_request)?
        .request(gloo_net::http::Method::PATCH, "api/domain")
        .await
}

pub async fn reader_data_set_provider(
    reader_request: ReaderRequest,
    token: Option<String>,
) -> Result<Box<dyn ReaderDataSetProvider + 'static + Send + Sync>, AppError> {
    println!("async_get_domain...");
    let response = fetch_reader_response(reader_request, token).await?;
    let provider = ResponseReaderDataSetProvider::new(response);
    Ok(Box::new(provider))
}

pub async fn response_reader_data_set_provider(
    name: impl IntoEntityName,
    condition: impl IntoConditionWhere,
    token: Option<String>,
) -> ResponseReaderDataSetProvider {
    let reader_request = ReaderRequest::new(
        name.into_entity_name(),
        ReaderContext::from_condition(condition.into_condition_where()),
    );
    let response = fetch_reader_response(reader_request, token).await.unwrap();
    ResponseReaderDataSetProvider::new(response)
}
