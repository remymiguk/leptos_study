use crate::app::{
    error::AppError,
    pagination::{Limit, Offset},
    repository::{get_repository, Repository, RepositoryProvider},
};
use chrono::NaiveDateTime;
use leptos::Scope;
use leptos::*;
use log::info;
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
pub struct ModelList<T: std::fmt::Debug + Clone + Send + Sync + PartialEq + 'static> {
    pub repository: Repository<T>,
    pub count: Resource<usize, Result<usize, ()>>,
    pub products: Resource<(usize, usize, usize), Result<Vec<T>, ()>>,
    pub page_read: ReadSignal<usize>,
    pub page_write: WriteSignal<usize>,
    pub max_page: Memo<usize>,
    pub list_reader: Memo<Option<Option<(Vec<T>, usize)>>>,
    pub version_write: WriteSignal<usize>,
    pub update_write: WriteSignal<Option<(WriteSignal<Option<Result<(), String>>>, T)>>,
}

impl<T: std::fmt::Debug + Clone + Send + Sync + PartialEq + 'static> ModelList<T> {
    pub fn new(cx: Scope) -> Self {
        let repository = get_repository::<T>().unwrap();

        let (version_read, version_write) = create_signal(cx, 0);

        let (page_read, page_write) = create_signal(cx, 1);

        let offset_read = move || page_read() - 1;

        let (limit_read, _limit_write) = create_signal(cx, 3);

        let count = {
            let repo = repository.clone();
            create_local_resource(cx, version_read, move |_| {
                let repo = repo.clone();
                async move { repo.count().await.map_err(|e| error!("{e}")) }
            })
        };

        let (update_read, update_write) = create_signal(
            cx,
            Option::<(WriteSignal<Option<Result<(), String>>>, T)>::None,
        );

        // @@@ add loading, error result
        let update_read = {
            let repo = repository.clone();
            create_local_resource(cx, update_read, move |payload| {
                let repo = repo.clone();
                async move {
                    if let Some((saved_write, payload)) = payload {
                        info!("inside update_read {payload:?}");
                        let result = repo
                            .update(cx, payload)
                            .await
                            .map_err(|e| {
                                error!("{e}");
                                e.to_string()
                            })
                            .map(|_| ());
                        version_write.update(|v| *v += 1);
                        saved_write.set(Some(result));
                    }
                }
            })
        };

        let products = {
            let repo = repository.clone();
            create_local_resource(
                cx,
                move || (version_read(), offset_read(), limit_read()),
                move |(_, offset, limit)| {
                    let repo = repo.clone();
                    async move {
                        repo.list(cx, Limit(limit), Offset(offset))
                            .await
                            .map_err(|e| error!("{e}"))
                    }
                },
            )
        };

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
            repository,
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
    pub async fn read(&self, cx: Scope, id: Uuid) -> Result<Option<T>, AppError> {
        self.repository.read(cx, id).await
    }

    pub async fn update(&mut self, cx: Scope, entity: T) -> Result<T, AppError> {
        self.version_write.update(|version| *version += 1);
        self.repository.update(cx, entity).await
    }
}
