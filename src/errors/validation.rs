use crate::errors::Error;

pub const CANNOT_FIND_QUERY_STRING_PARAMETERS: Error = Error {
    code: "CANNOT_FIND_QUERY_STRING_PARAMETERS",
    message: "Can not find queryStringParameters parameter.",
};

pub const CANNOT_FIND_QUERY: Error = Error {
    code: "CANNOT_FIND_QUERY",
    message: "Can not find query parameter.",
};
