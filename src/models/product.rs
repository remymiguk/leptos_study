use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Product {
    pub id: Uuid,
    pub description: String,
    pub category: Uuid,
    pub price: Decimal,
    pub created_at: NaiveDateTime,
}
