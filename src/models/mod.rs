pub mod header;

// ReExports
pub use header::*;

use juniper::GraphQLInputObject;
use serde::Serialize;
use std::fmt;

/// ページネーション
#[derive(Debug, Clone, GraphQLInputObject, Serialize)]
pub struct ArgPagination {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl Default for ArgPagination {
    fn default() -> Self {
        Self {
            limit: None,
            offset: None,
        }
    }
}

///
/// デフォルトは desc
/// sort: desc
///
#[derive(Debug, Clone, GraphQLInputObject, Serialize)]
pub struct ArgOrderBy {
    pub order: String,
    pub by: String,
}

#[derive(Debug, Clone)]
pub enum Order {
    Desc,
    Asc,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

///
/// DB のテーブルの取得モデル ( Fetch ) に対して実装するメソッド
///
pub trait Model<R, E> {
    fn to_res(&self) -> Result<R, E>;
}

///
/// DB のテーブルの作成モデル ( New ) に対して実装するメソッド
///
pub trait NewModel<M, N, E> {
    fn from_model(model: &M) -> Result<N, E>;
    fn to_model(&self) -> Result<M, E>;
    fn insert(&self) -> Result<M, E>;

    ///
    /// update する際は、API の引数ですべてを受け取り、その受け取った引数から NewModel を作成して update
    fn update(&self) -> Result<M, E>;
}

///
/// Model のデータを返すための Model
///
pub trait ResponseModel {}

#[derive(Debug, Clone, Serialize)]
pub struct ResPagination {
    pub limit: String,
    pub offset: String,
    pub total: String,
}
