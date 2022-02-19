use thiserror::Error;

#[derive(Error, Clone, Debug, PartialEq)]
pub enum Error { 
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
    RequestError
}
