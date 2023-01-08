use crate::{
    api,
    app::{
        error::AppError,
        pagination::{Limit, Offset},
        repository::Repository,
    },
    models::product::Product,
};
use async_trait::async_trait;
use core::fmt::Debug;
use leptos::Scope;
use uuid::Uuid;

pub fn products(path: &str) -> String {
    format!("http://127.0.0.1:8080/api/products{path}")
}

#[derive(Debug)]
pub struct ApiProductRepository {}

#[async_trait(?Send)]
impl Repository for ApiProductRepository {
    type Entity = Product;

    async fn read(&self, cx: Scope, id: Uuid) -> Result<Option<Self::Entity>, AppError> {
        api::fetch_api::<Product>(cx, &products(&format!("/{id}"))).await
    }

    async fn list(
        &self,
        cx: Scope,
        limit: Limit,
        offset: Offset,
    ) -> Result<Vec<Self::Entity>, AppError> {
        let path = format!("?offset={}&limit={}", offset.0, limit.0);
        let list = api::fetch_api::<Vec<Product>>(cx, &products(&path))
            .await?
            .unwrap_or_default();
        Ok(list)
    }

    async fn create(&self, cx: Scope, entity: Self::Entity) -> Result<Self::Entity, AppError> {
        todo!()
    }

    async fn update(&self, cx: Scope, entity: Self::Entity) -> Result<Self::Entity, AppError> {
        todo!()
    }

    async fn delete(&self, cx: Scope, id: Uuid) -> Result<Uuid, AppError> {
        todo!()
    }
}
