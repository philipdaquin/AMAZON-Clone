
use chrono::NaiveDate;
use diesel::{Insertable, AsChangeset};
use juniper::{GraphQLObject, GraphQLInputObject};
use serde::Deserialize;
use crate::schema::sales;
use crate::schema::sale_products;
use super::sale_resolver::*;
use crate::graphql_modules::price::price_types::*;
use crate::graphql_modules::products::product_types::*;



#[derive(Identifiable, Associations, Serialize,
    Queryable, Debug, Clone, PartialEq, GraphQLObject )]
#[table_name = "sale_products"]
#[belongs_to(Sale)]
#[belongs_to(Product)]
pub struct ProductSaleInfo { 
    pub id: i32,
    pub product_id: i32,
    pub sale_id: i32, 
    pub amount: f64, 
    pub discount: i32, 
    pub tax: i32, 
    pub price: i32,
    pub total: f64
}
#[derive(Clone, GraphQLObject, Debug)]
pub struct ProductForSale  {
    pub sale_info: ProductSaleInfo,
    pub product: Product
}
#[derive(Insertable, Deserialize, Serialize, AsChangeset, 
    Debug, Clone, PartialEq, GraphQLInputObject)]
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
#[derive(Clone, Debug, GraphQLInputObject)]
pub struct NewProductSaleInfo  {
    pub sale_info: NewProductSale,
    pub product: NewProduct
}
#[derive(GraphQLInputObject)]
pub struct NewSaleProducts { 
    pub data: Vec<NewProductSaleInfo>
}


/// Sales 
#[derive(Identifiable, Queryable, Debug,  Clone, PartialEq, GraphQLObject)]
#[table_name = "sales"]
#[graphql(description = "Sales Object")]
pub struct Sale { 
    pub id: i32, 
    pub user_id: i32,
    pub sale_date: NaiveDate,
    pub total: f64,
    pub bill_number: Option<String>

}
#[derive(Insertable, Deserialize, GraphQLInputObject,
Serialize, AsChangeset, Debug, Clone, PartialEq)]
#[table_name = "sales"]
pub struct NewSale { 
    pub id: Option<i32>, 
    pub user_id: Option<i32>,
    pub sale_date: Option<NaiveDate>,
    pub total: Option<f64>,
    pub bill_number: Option<String>

}
#[derive(Debug, Clone, GraphQLObject)]
pub struct FullSale { 
    pub sale_info: Sale,
    pub sales_products: Vec<ProductForSale>
}
#[derive(Debug, Clone)]
pub struct FullNewSale { 
    pub sale: NewSale,
    pub sales_products: Vec<NewProductSaleInfo>
}
#[derive(Debug, Clone, GraphQLObject)]
pub struct ListSale { 
    pub data: Vec<FullSale>
}