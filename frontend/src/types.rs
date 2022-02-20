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