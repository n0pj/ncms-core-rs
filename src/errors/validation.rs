use crate::errors::Error;

pub enum ValidationError {
    CannotFindQueryStringParameters,
    CannotFindQuery,
    CannotFindHeaders,
}
