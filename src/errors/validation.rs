use crate::errors::Error;

pub const CANNOT_FIND_QUERY_STRING_PARAMETERS: Error = Error {
    property: "queryStringParameters",
    code: "CANNOT_FIND_QUERY_STRING_PARAMETERS",
    message: "Can not find queryStringParameters parameter.",
};

pub const CANNOT_FIND_QUERY: Error = Error {
    property: "query",
    code: "CANNOT_FIND_QUERY",
    message: "Can not find query parameter.",
};

pub const CANNOT_FIND_HEADERS: Error = Error {
    property: "headers",
    code: "CANNOT_FIND_HEADERS",
    message: "Can not find headers.",
};
