use std::fmt::{Display, Formatter, Result};
use yew::prelude::*;


#[derive(Debug, PartialEq, Clone, Default)]
pub struct ProductType { 
    pub id: i32,
    pub title: String,
    pub image: String, 
    pub rating: usize,
    pub price: f32
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CartProduct { 
    pub product: ProductType,
    pub quantity: i32
}

impl Display for ProductType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

