use gloo::storage::{LocalStorage, Storage};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use dotenv::dotenv;
use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use crate::error::Error;


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
) -> Result<T, Error> where 
        T: DeserializeOwned + 'static + Debug,
        B: Serialize + Debug ,
    {



}