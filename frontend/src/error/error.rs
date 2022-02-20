use thiserror::Error as Errors;

use crate::types::ErrorInfo;

#[derive(Errors, Clone, Debug, PartialEq)]
pub enum ServiceError { 
    #[error("UnAuthorised Access")]
    UnAuthorised, 

    #[error("Forbidden")]
    Forbidden,

    #[error("Not Found")]
    NotFound,

    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Deserialize Error")]
    DeserializeError,

    #[error("Http Request Error")]
    RequestError,

    #[error("Unprocessable Entity: {0:?}")]
    UnprocessableEntity(ErrorInfo)
}
