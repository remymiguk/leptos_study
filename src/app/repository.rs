use super::{
    error::AppError,
    pagination::{Limit, Offset},
};
use async_trait::async_trait;
use dyn_clonable::{clonable, dyn_clone};
use leptos::Scope;
use std::clone::Clone;
use std::{any::Any, sync::Mutex};
use uuid::Uuid;

#[clonable]
pub trait RepositoryClonable: Clone + std::fmt::Debug {}

impl<T> RepositoryClonable for T where T: Clone + std::fmt::Debug {}

/// Trait that defines common repository operations.
#[async_trait(?Send)]
pub trait RepositoryProvider: RepositoryClonable {
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

#[derive(Debug)]
pub struct Repository<T: std::fmt::Debug + Clone + Send + Sync + 'static> {
    repository: Box<dyn RepositoryProvider<Entity = T> + Send + Sync + 'static>,
}

impl<T: std::fmt::Debug + Clone + Send + Sync + 'static> Repository<T> {
    pub fn new(repository: impl RepositoryProvider<Entity = T> + Send + Sync + 'static) -> Self {
        Self {
            repository: Box::new(repository),
        }
    }
}

impl<T: std::fmt::Debug + Clone + Send + Sync + 'static> Clone for Repository<T> {
    fn clone(&self) -> Self {
        Self {
            repository: dyn_clone::clone_box(&*self.repository),
        }
    }
}

#[async_trait(?Send)]
impl<T: std::fmt::Debug + Clone + Send + Sync + 'static> RepositoryProvider for Repository<T> {
    type Entity = T;

    async fn read(&self, cx: Scope, id: Uuid) -> Result<Option<Self::Entity>, AppError> {
        self.read(cx, id).await
    }

    async fn list(
        &self,
        cx: Scope,
        limit: Limit,
        offset: Offset,
    ) -> Result<Vec<Self::Entity>, AppError> {
        self.list(cx, limit, offset).await
    }

    async fn count(&self) -> Result<usize, AppError> {
        self.count().await
    }

    async fn create(&self, cx: Scope, entity: Self::Entity) -> Result<Self::Entity, AppError> {
        self.create(cx, entity).await
    }

    async fn update(&self, cx: Scope, entity: Self::Entity) -> Result<Self::Entity, AppError> {
        self.update(cx, entity).await
    }

    async fn delete(&self, cx: Scope, id: Uuid) -> Result<Uuid, AppError> {
        self.delete(cx, id).await
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

static REPOSITORIES: Mutex<Vec<Box<dyn Any + 'static + Send + Sync>>> = Mutex::new(vec![]);

pub fn add_repository_provider<T: std::fmt::Debug + Clone + 'static + Send + Sync>(
    repository_provider: impl RepositoryProvider<Entity = T> + 'static + Send + Sync,
) {
    let repository = Repository::new(repository_provider);
    add_repository(repository);
}

pub fn add_repository<T: std::fmt::Debug + Clone + 'static + Send + Sync>(
    repository: Repository<T>,
) {
    let mut repos = REPOSITORIES.lock().unwrap();
    repos.push(Box::new(repository));
}

pub fn get_repository<T>() -> Option<Repository<T>>
where
    T: std::fmt::Debug + Clone + Send + Sync + 'static,
{
    let repos = REPOSITORIES.lock().unwrap();
    for repo in &**repos {
        if let Some(repo) = repo.downcast_ref::<Repository<T>>() {
            return Some(repo.clone());
        }
    }
    None
}
