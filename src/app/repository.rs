use crate::models::product::Product;

use super::{
    error::AppError,
    pagination::{Limit, Offset},
};
use async_trait::async_trait;
use leptos::Scope;
use once_cell::sync::OnceCell;
use uuid::Uuid;

/// A trait that defines common repository operations
#[async_trait(?Send)]
pub trait Repository: std::fmt::Debug {
    type Entity;

    async fn read(&self, cx: Scope, id: Uuid) -> Result<Option<Self::Entity>, AppError>;

    async fn list(
        &self,
        cx: Scope,
        limit: Limit,
        offset: Offset,
    ) -> Result<Vec<Self::Entity>, AppError>;

    async fn create(&self, cx: Scope, entity: Self::Entity) -> Result<Self::Entity, AppError>;

    async fn update(&self, cx: Scope, entity: Self::Entity) -> Result<Self::Entity, AppError>;

    async fn delete(&self, cx: Scope, id: Uuid) -> Result<Uuid, AppError>;
}

static PRODUCT_REPOSITORY_INSTANCE: OnceCell<
    Box<dyn Repository<Entity = Product> + 'static + Send + Sync>,
> = OnceCell::new();

pub fn set_product_repository(
    repository: impl Repository<Entity = Product> + 'static + Send + Sync + std::fmt::Debug,
) {
    PRODUCT_REPOSITORY_INSTANCE
        .set(Box::new(repository))
        .unwrap();
}

pub fn product_repository() -> &'static (dyn Repository<Entity = Product> + 'static + Send + Sync) {
    &**PRODUCT_REPOSITORY_INSTANCE.get().unwrap()
}
