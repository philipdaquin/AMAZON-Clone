use std::{fmt::{Display, Formatter, Result}};
use yew::prelude::*;


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

impl Display for ProductType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self { 
            id => write!(f, "{:?}", self.id),
            title => write!(f, "{:?}", self.title),
            image => write!(f, "{:?}", self.image),
            rating => write!(f, "{:?}", self.rating),
            price => write!(f, "{:?}", self.price),
        }
    }
}

