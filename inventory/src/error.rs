use r2d2::Error as ConnectionManagerError;
use actix_web::{
    error::ResponseError, HttpResponse
};
use tokio::task::JoinError;
use uuid::Error as ParseError;
use diesel::result::{ Error as DatabaseRequestError};
use actix_threadpool::BlockingError;
use thiserror::Error;
use std::{convert::From, fmt::Debug };


#[derive(Debug, Error)]
pub enum ServerError { 
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Bad User Request: {0}")] //    <-----
    BadRequest(String),

    #[error("UnAuthorised User Request")]
    UnAuthorised,

    #[error("Database Request Error")]
    DatabaseRequestError(#[from] DatabaseRequestError),

    #[error("Connection Pool Manager Error {0}")]
    ConnectionManagerError(#[from] ConnectionManagerError),

    #[error("Join Error {0}")]
    JoinError(#[from] JoinError),

    #[error("Blocking Error: {0}")]  //   <-----
    BlockingError(String)
}

impl ResponseError for ServerError { 
    fn error_response(&self) -> HttpResponse {
        match self { 
            _ => { 
                HttpResponse::InternalServerError()
                    .json("Interna Server Error, Please Try Again later")
            },
            ServerError::BadRequest(ref message) => { 
                HttpResponse::BadRequest()
                    .json(message)
            },
            ServerError::UnAuthorised => { 
                HttpResponse::Unauthorized()
                    .json("UnAuthorised Request!")
            }
        }
    }
}
//  Convert Badrequest to String
impl From<ParseError> for ServerError { 
    fn from(_: ParseError) -> Self { 
        ServerError::BadRequest("Invalid UUId".into())
    }
}
// Convert BlockingError to String
//  BlockingError can be both: Error<T>, Canceled
impl<T: Debug> From<BlockingError<T>> for ServerError { 
    fn from(from: BlockingError<T>) -> Self { 
        ServerError::BlockingError(from.to_string())
    }
}


