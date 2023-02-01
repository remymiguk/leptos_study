use crate::app::{
    error::AppError,
    pagination::{Limit, Offset},
    repository::product_repository,
};
use chrono::NaiveDateTime;
use leptos::Scope;
use leptos::*;
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

#[derive(Clone, Debug)]
pub struct ProductModel {
    pub count: Resource<usize, Result<usize, ()>>,
    pub products: Resource<(usize, usize, usize), Result<Vec<Product>, ()>>,
    pub page_read: ReadSignal<usize>,
    pub page_write: WriteSignal<usize>,
    pub max_page: Memo<usize>,
    pub list_reader: Memo<Option<Option<(Vec<Product>, usize)>>>,
    pub version_write: WriteSignal<usize>,
    pub update_write: WriteSignal<Option<Product>>,
}

impl ProductModel {
    pub fn new(cx: Scope) -> Self {
        let (version_read, version_write) = create_signal(cx, 0);

        let (page_read, page_write) = create_signal(cx, 1);

        let offset_read = move || page_read() - 1;

        let (limit_read, _limit_write) = create_signal(cx, 3);

        let count = create_local_resource(cx, version_read, move |_| async move {
            product_repository()
                .count()
                .await
                .map_err(|e| error!("{e}"))
        });

        let (update_read, update_write) = create_signal(cx, Option::<Product>::None);

        // @@@ add loading, error result
        let update_read = create_local_resource(cx, update_read, move |payload| async move {
            if let Some(payload) = payload {
                product_repository()
                    .update(cx, payload)
                    .await
                    .map_err(|e| error!("{e}"))
                    .unwrap();
                version_write.update(|v| *v += 1);
            };
        });

        let products = create_local_resource(
            cx,
            move || (version_read(), offset_read(), limit_read()),
            move |(_, offset, limit)| async move {
                product_repository()
                    .list(cx, Limit(limit), Offset(offset))
                    .await
                    .map_err(|e| error!("{e}"))
            },
        );

        // Calc max page
        let max_page = create_memo(cx, move |_| match count.read() {
            Some(Ok(count)) => (count as f32 / limit_read() as f32).ceil() as usize,
            _ => 0,
        });

        let list_reader = create_memo(cx, move |_| {
            match (products.loading()(), products.read(), count.read()) {
                (false, Some(Ok(products)), Some(Ok(count))) => Some(Some((products, count))),
                (true, Some(Err(_)), _) | (true, _, Some(Err(_))) => Some(None),
                _ => None,
            }
        });

        Self {
            count,
            products,
            page_read,
            page_write,
            max_page,
            list_reader,
            version_write,
            update_write,
        }
    }

    // @@@ EXPORT read as signal
    pub async fn read(&self, cx: Scope, id: Uuid) -> Result<Option<Product>, AppError> {
        product_repository().read(cx, id).await
    }

    pub async fn update(&mut self, cx: Scope, entity: Product) -> Result<Product, AppError> {
        self.version_write.update(|version| *version += 1);
        product_repository().update(cx, entity).await
    }
}
