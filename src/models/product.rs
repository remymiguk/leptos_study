use chrono::NaiveDateTime;
use leptos::Scope;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::app::{error::AppError, repository::product_repository};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Product {
    pub id: Uuid,
    pub description: String,
    pub category: Uuid,
    pub price: Decimal,
    pub created_at: NaiveDateTime,
}

#[derive(Clone, Debug)]
pub struct ProductModel {}

impl Default for ProductModel {
    fn default() -> Self {
        Self::new()
    }
}

impl ProductModel {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn read(&self, cx: Scope, id: Uuid) -> Result<Option<Product>, AppError> {
        product_repository().read(cx, id).await
    }
}
