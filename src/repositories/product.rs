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
use log::info;
use std::{
    cell::RefCell,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

static API_ROOT: &str = "http://127.0.0.1:8080/api";

#[derive(Debug)]
pub struct ApiProductRepository {
    root_url: String,
}

impl ApiProductRepository {
    pub fn new() -> Self {
        Self {
            root_url: API_ROOT.to_string(),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}/products{path}", self.root_url)
    }
}

impl Default for ApiProductRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl Repository for ApiProductRepository {
    type Entity = Product;

    async fn read(&self, cx: Scope, id: Uuid) -> Result<Option<Self::Entity>, AppError> {
        api::fetch_api::<Product>(cx, &self.url(&format!("/{id}"))).await
    }

    async fn list(
        &self,
        cx: Scope,
        limit: Limit,
        offset: Offset,
    ) -> Result<Vec<Self::Entity>, AppError> {
        let path = format!("?offset={}&limit={}", offset.0, limit.0);
        let list = api::fetch_api::<Vec<Product>>(cx, &self.url(&path))
            .await?
            .unwrap_or_default();
        Ok(list)
    }

    async fn count(&self) -> Result<usize, AppError> {
        Ok(5)
    }

    async fn create(&self, _cx: Scope, _entity: Self::Entity) -> Result<Self::Entity, AppError> {
        todo!()
    }

    async fn update(&self, _cx: Scope, _entity: Self::Entity) -> Result<Self::Entity, AppError> {
        todo!()
    }

    async fn delete(&self, _cx: Scope, _id: Uuid) -> Result<Uuid, AppError> {
        todo!()
    }
}

static MOCK_DIR: &str = "./resources/";
static MOCK_FILE: &str = "products.json";

#[derive(Debug)]
pub struct MockProductRepository {
    products: Vec<Product>,
}

impl MockProductRepository {
    pub fn new() -> Self {
        let path = PathBuf::from(MOCK_DIR);
        let content = fs::read_to_string(path.join(MOCK_FILE)).unwrap();
        let products: Vec<Product> = serde_json::from_str(&content).unwrap();
        Self { products }
    }
}

impl Default for MockProductRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl Repository for MockProductRepository {
    type Entity = Product;

    async fn read(&self, _cx: Scope, id: Uuid) -> Result<Option<Self::Entity>, AppError> {
        Ok(self.products.iter().find(|p| p.id == id).cloned())
    }

    async fn list(
        &self,
        _cx: Scope,
        limit: Limit,
        offset: Offset,
    ) -> Result<Vec<Self::Entity>, AppError> {
        info!("*** repository offset {} limit {}", offset.0, limit.0);
        let list = self
            .products
            .iter()
            .skip(offset.0 * limit.0)
            .take(limit.0)
            .cloned()
            .collect::<Vec<_>>();
        Ok(list)
    }

    async fn count(&self) -> Result<usize, AppError> {
        Ok(self.products.len())
    }

    async fn create(&self, _cx: Scope, _entity: Self::Entity) -> Result<Self::Entity, AppError> {
        todo!()
    }

    async fn update(&self, _cx: Scope, _entity: Self::Entity) -> Result<Self::Entity, AppError> {
        todo!()
    }

    async fn delete(&self, _cx: Scope, _id: Uuid) -> Result<Uuid, AppError> {
        todo!()
    }
}

static PRODUCTS_BUFFER: &str = r#"
[
    {
        "id": "f92a6aa9-8fde-4e1b-9fc2-36e8a7f9cad3",
        "description": "Smart TV LED 32\" LG 32LQ620BPSB",
        "category": "38b9bce7-42cd-49cd-b8de-024eb4380c07",
        "price": "300.9900",
        "created_at": "2023-01-02T00:02:14.129178"
    },
    {
        "id": "f92a6aa9-8fde-4e1b-9fc2-d336e8a7f9ca",
        "description": "Smart TV LED 47\" LG 47LQ620BPSB",
        "category": "38b9bce7-42cd-49cd-b8de-024eb4380c07",
        "price": "330.9900",
        "created_at": "2023-01-02T00:02:14.129178"
    },
    {
        "id": "2f2b62a9-1fb8-48ae-8bfb-0e12740500f6",
        "description": "Smart TV LED 4K UHD 50\" Samsung UN50AU7700GXZD",
        "category": "38b9bce7-42cd-49cd-b8de-024eb4380c07",
        "price": "650",
        "created_at": "2023-01-02T00:02:14.129178"
    },
    {
        "id": "dfff3c2c-2ce8-4724-87f6-71f5fc8597c9",
        "description": "Smart TV Android LED Full HD 43\" Philips 43PFG6917/78",
        "category": "38b9bce7-42cd-49cd-b8de-024eb4380c07",
        "price": "650",
        "created_at": "2023-01-02T00:02:14.129178"
    },
    {
        "id": "2f2b62a9-1fb8-48ae-8bfb-f60e12740500",
        "description": "Smart TV LED 4K UHD 47\" Samsung UN47AU7700GXZD",
        "category": "38b9bce7-42cd-49cd-b8de-024eb4380c07",
        "price": "550",
        "created_at": "2023-01-02T00:02:14.129178"
    }
]
"#;

#[derive(Debug)]
pub struct BufferProductRepository {
    products: Arc<Mutex<Vec<Product>>>,
}

impl BufferProductRepository {
    pub fn new() -> Self {
        let products: Vec<Product> = serde_json::from_str(PRODUCTS_BUFFER).unwrap();
        Self {
            products: Arc::new(Mutex::new(products)),
        }
    }
}

impl Default for BufferProductRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl Repository for BufferProductRepository {
    type Entity = Product;

    async fn read(&self, _cx: Scope, id: Uuid) -> Result<Option<Self::Entity>, AppError> {
        Ok(self
            .products
            .lock()
            .unwrap()
            .iter()
            .find(|p| p.id == id)
            .cloned())
    }

    async fn list(
        &self,
        _cx: Scope,
        limit: Limit,
        offset: Offset,
    ) -> Result<Vec<Self::Entity>, AppError> {
        info!("*** repository offset {} limit {}", offset.0, limit.0);
        let list = self
            .products
            .lock()
            .unwrap()
            .iter()
            .skip(offset.0 * limit.0)
            .take(limit.0)
            .cloned()
            .collect::<Vec<_>>();
        Ok(list)
    }

    async fn count(&self) -> Result<usize, AppError> {
        Ok(self.products.lock().unwrap().len())
    }

    async fn create(&self, _cx: Scope, _entity: Self::Entity) -> Result<Self::Entity, AppError> {
        todo!()
    }

    async fn update(&self, _cx: Scope, entity: Self::Entity) -> Result<Self::Entity, AppError> {
        let mut list = self.products.lock().unwrap();
        if let Some(e) = list.iter_mut().find(|p| p.id == entity.id) {
            *e = entity.clone();
        };
        Ok(entity)
    }

    async fn delete(&self, _cx: Scope, _id: Uuid) -> Result<Uuid, AppError> {
        todo!()
    }
}
