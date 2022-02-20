use gloo::storage::{LocalStorage, Storage};
use gloo_console::log;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use dotenv_codegen;
use crate::types::ErrorInfo;
use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use crate::error::ServiceError;


// TODO 
const API_ROOT: &str = dotenv!("API_ROOT");
const TOKEN_KEY: &str = "token";
lazy_static! { 
    //  JWT read from local storage 
    pub static ref TOKEN: RwLock<Option<String>> = { 
        if let Ok(token) = LocalStorage::get(TOKEN_KEY) { 
            RwLock::new(Some(token))
        } else { 
            RwLock::new(None)
        }
    };
}
//  Set jwt token to local storage 
pub fn set_token(token: Option<String>) { 
    if let Some(token) = token.clone() { 
        LocalStorage::set(TOKEN_KEY, token).expect("Failed to Set token");
    } else { 
        let mut token_lock = TOKEN.write();
        *token_lock = token
    }
}

//  Get jwt from lazy static
pub fn get_token() -> Option<String> { 
    let token_lock = TOKEN.read();
    token_lock.clone()
}
pub async fn request<B, T>(
    method: Method,
    url: String, 
    body: B
) -> Result<T, ServiceError> where 
        T: DeserializeOwned + 'static + Debug,
        B: Serialize + Debug  {
            let allowed_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
            let url = format!("{}{}", API_ROOT, url);
            let mut builder = reqwest::Client::new()
                .request(method, url)
                .header("Content-Type", "application/json");
            if let Some(token) = get_token() { 
                builder = builder.bearer_auth(token)
            }

            if allowed_body { 
                builder = builder.json(&body)
            }
            
            let response = builder.send().await;

            if let Ok(data) = response {
                if data.status().is_success() {
                    let data: Result<T, _> = data.json::<T>().await;
                    if let Ok(data) = data {
                        log::debug!("Response: {:?}", data);
                        Ok(data)
                    } else {
                        Err(ServiceError::DeserializeError)
                    }
                } else {
                    match data.status().as_u16() {
                        401 => Err(ServiceError::UnAuthorised),
                        403 => Err(ServiceError::Forbidden),
                        404 => Err(ServiceError::NotFound),
                        500 => Err(ServiceError::InternalServerError),
                        422 => {
                            let data: Result<ErrorInfo, _> = data.json::<ErrorInfo>().await;
                            if let Ok(data) = data {
                                Err(ServiceError::UnprocessableEntity(data))
                            } else {
                                Err(ServiceError::DeserializeError)
                            }
                        }
                        _ => Err(ServiceError::RequestError),
                    }
                }
            } else {
                Err(ServiceError::RequestError)
            }
}
//  Delete Request
pub async fn request_delete<T>(url: String) -> Result<T, ServiceError> 
where T: DeserializeOwned  + 'static +  Debug, { 
    request(reqwest::Method::DELETE, url, ()).await
}
//  Get Request 
//  Delete Request
pub async fn request_get<T>(url: String) -> Result<T, ServiceError> 
where T: DeserializeOwned  + 'static +  Debug, { 
    request(reqwest::Method::GET, url, ()).await
}
//  Get Request 
//  Delete Request
pub async fn request_post<B, T>(url: String, body: B) -> Result<T, ServiceError>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(reqwest::Method::POST, url, body).await
}

//  Delete Request
pub async fn request_put<T, B>(url: String, body: B) -> Result<T, ServiceError> 
where T: DeserializeOwned  + 'static +  Debug,
    B: Serialize + Debug
{ 
    request(reqwest::Method::PUT, url, body).await
}
pub fn limit(count: u32, p: u32) -> String { 
    let offset = if p > 0  { 
        p * count 
    } else { 0};
    format!("limit={}&offset={}", count, offset)
}
