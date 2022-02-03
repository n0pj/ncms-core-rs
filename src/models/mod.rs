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
/// DB のテーブルの作成モデル ( Fetch ) に対して実装するメソッド
///
pub trait ModelMethods<R> {
    fn to_res(&self) -> R;
}

///
/// DB のテーブルの作成モデル ( New ) に対して実装するメソッド
///
pub trait NewModelMethods<M, R> {
    fn save(&self) -> M;
    fn to_model(&self) -> M;
    fn to_res(&self) -> R;
}
