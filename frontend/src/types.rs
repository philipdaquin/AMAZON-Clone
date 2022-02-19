use std::{fmt::{Display, Formatter, Result}};
use yew::prelude::*;

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
#[derive(Clone, Debug, Default)]
pub struct LoginInfo {
    pub email: String, 
    pub password: String 
}
#[derive(Clone, Debug, Default)]
pub struct LoginInfoWrapper { 
    pub user: LoginInfo
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct UserInfo  { 
    pub email: String, 
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String> 
}

