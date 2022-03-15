
use diesel::{Insertable, AsChangeset};
use serde::Deserialize;

use crate::models::product::{Product, NewProduct};
use crate::schema::sale_products;
use crate::models::sales::Sale;

#[derive(Identifiable, Associations, Queryable, Debug, Clone, PartialEq)]
#[table_name = "sale_products"]
#[belongs_to(Sale)]
#[belongs_to(Product)]
pub struct ProductSaleInfo { 
    pub id: i32,
    pub product_id: i32,
    pub sale_id: i32, 
    pub amount: i32, 
    pub discount: i32, 
    pub tax: i32, 
    pub price: i32,
    pub total: i32
}
pub struct ProductForSale  {
    pub sale_info: ProductSaleInfo,
    pub product: Product
}
#[derive(Insertable, Deserialize, Serialize, AsChangeset, Debug, Clone, PartialEq)]
#[table_name = "sale_products"]
pub struct NewProductSale { 
    pub id: Option<i32>,
    pub product_id: Option<i32>,
    pub sale_id: Option<i32>, 
    pub amount: Option<f64>, 
    pub discount: Option<i32>, 
    pub tax: Option<i32>, 
    pub price: Option<i32>,
    pub total: Option<f64>,
}

pub struct NewProductSaleInfo  {
    pub sale_info: NewProductSale,
    pub product: NewProduct
}

pub struct NewSaleProducts { 
    pub data: Vec<NewProductSaleInfo>
}