use std::{any::Any, sync::Mutex};

use super::{
    error::AppError,
    pagination::{Limit, Offset},
};
use crate::{models::product::Product, repositories::product::BufferProductRepository};
use async_trait::async_trait;
use leptos::Scope;
use once_cell::sync::OnceCell;
use uuid::Uuid;

/// Trait that defines common repository operations.
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

    async fn count(&self) -> Result<usize, AppError>;

    async fn create(&self, cx: Scope, entity: Self::Entity) -> Result<Self::Entity, AppError>;

    async fn update(&self, cx: Scope, entity: Self::Entity) -> Result<Self::Entity, AppError>;

    async fn delete(&self, cx: Scope, id: Uuid) -> Result<Uuid, AppError>;

    fn as_any(&self) -> &dyn Any;
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

static REPOSITORIES: OnceCell<Box<dyn Any + 'static + Send + Sync>> = OnceCell::new();

pub fn add_repository(
    repository: impl Repository<Entity = Product> + 'static + Send + Sync + std::fmt::Debug,
) {
    let repos = REPOSITORIES.lock().unwrap();
    repos.push(Box::new(repository));
}

#[test]
fn test() {
    let repo: Box<dyn Repository<Entity = _> + 'static + Send + Sync> =
        Box::new(BufferProductRepository::new());

    let any = (&*repo).as_any();

    if let Some(repo) = any.downcast_ref::<&dyn Repository<Entity = Product>>() {
        println!("repo!");
    }
}
