use std::{fmt::{Display, Formatter, Result}};
use serde::{Serialize, Deserialize};
use yew::prelude::*;
use std::collections::HashMap;
// Products 
#[derive(PartialEq, Properties, Clone)]
pub struct ProductType { 
    pub id: i32,
    pub title: String,
    pub image: String, 
    pub rating: usize,
    pub price: f32
}

#[derive(PartialEq, Properties, Clone)]
pub struct CartProduct { 
    pub product: ProductType,
    pub quantity: i32
}
//  User Login
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoginInfo {
    pub email: String, 
    pub password: String 
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoginInfoWrapper { 
    pub user: LoginInfo
}
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserInfo  { 
    pub email: String, 
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String> 
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorInfo { 
    pub errors: HashMap<String, Vec<String>>
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInfoWrapper { 
    pub user_info: UserInfo,   
}



#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserUpdateInfo {
    pub email: String,
    pub username: String,
    pub password: Option<String>,
    pub image: String,
    pub bio: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserUpdateInfoWrapper {
    pub user: UserUpdateInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegisterInfoWrapper {
    pub user: RegisterInfo,
}

impl UserInfo {
    pub fn is_authenticated(&self) -> bool {
        !self.token.is_empty()
    }
}
