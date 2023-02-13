use crate::app::error::AppError;
use crate::services::api_request_gloo::APIRequestGloo;
use voxi_dataset::{
    data_sets::records_buffer::RecordsBuffer,
    domain::{
        reader_data_reader_provider::ReaderDataSetProvider, reader_request::ReaderRequest,
        reader_response::ReaderResponse,
        response_reader_data_set_provider::ResponseReaderDataSetProvider,
    },
    writer::writer_request::WriterRequest,
};

pub async fn fetch_writer_response(
    writer_request: WriterRequest,
    token: Option<String>,
) -> Result<ReaderResponse, AppError> {
    println!("async_get_domain...");
    APIRequestGloo::new()
        .token_opt(token)
        .body(writer_request)?
        .request(gloo_net::http::Method::PATCH, "api/update")
        .await
}

pub async fn writer_data_set_provider(
    writer_request: WriterRequest,
    token: Option<String>,
) -> Result<Box<dyn ReaderDataSetProvider + 'static + Send + Sync>, AppError> {
    println!("async_get_domain...");
    let response = fetch_writer_response(writer_request, token).await?;
    let provider = ResponseReaderDataSetProvider::new(response);
    Ok(Box::new(provider))
}

pub async fn response_writer_data_set_provider(
    reader_request: ReaderRequest,
    records_buffer: RecordsBuffer,
    token: Option<String>,
) -> ResponseReaderDataSetProvider {
    let mut writer_request = WriterRequest::new(reader_request);
    writer_request = writer_request.with_records_buffer(records_buffer);
    let response = fetch_writer_response(writer_request, token).await.unwrap();
    ResponseReaderDataSetProvider::new(response)
}
