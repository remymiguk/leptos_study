use serde::Deserialize;

// The query parameters
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<Offset>,
    pub limit: Option<Limit>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Offset(pub usize);

#[derive(Debug, Deserialize, Default)]
pub struct Limit(pub usize);
