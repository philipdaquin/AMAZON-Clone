use crate::schema::{prices, prices_products};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, Connection, PgConnection};
use diesel::{result::Error as DbError};
use itertools::Itertools;
use crate::models::product::Product;
use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};


#[derive(Queryable, Debug, Clone, GraphQLObject,
    Identifiable, Serialize, Deserialize, PartialEq)]
#[table_name = "prices"]
pub struct Price { 
    pub id: i32, 
    pub name: String,
    pub user_id: i32
}
#[derive(Deserialize, Serialize, GraphQLObject, Clone)]
pub struct ListedPrice { 
    pub data: Vec<Price>
}
#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
pub struct ProductPriceInfo { 
    pub price_info: PriceInfo, 
    pub price: Price
}

#[derive(Debug, Clone, Serialize, Deserialize, 
    GraphQLInputObject, PartialEq, AsChangeset, Insertable)]
#[table_name = "prices"]
pub struct NewPriceForm { 
    pub id: Option<i32>,
    pub name: Option<String>,
    pub user_id: Option<i32>
}

#[derive(Queryable, Debug, Clone, GraphQLObject, 
    Associations, Identifiable, Serialize, Deserialize, PartialEq)]
#[belongs_to(Price)]
#[belongs_to(Product)]
#[table_name="prices_products"]
pub struct PriceInfo {
    pub id: i32, 
    pub price_id: i32,
    pub product_id: i32,
    pub user_id: i32,
    pub amount: Option<i32>
}
#[derive(Serialize, Deserialize, Clone, GraphQLInputObject,
    Debug, Insertable, AsChangeset)]
#[table_name = "prices_products"]
pub struct FormPriceInfo { 
    pub id: Option<i32>, 
    pub price_id: i32,
    pub product_id: Option<i32>,
    pub user_id: Option<i32>,
    pub amount: Option<i32>
}
#[derive(Debug, Clone, Serialize, Deserialize, GraphQLInputObject)]
pub struct ProductPriceInfoUpdate { 
    pub updated_price_info: FormPriceInfo,
    pub to_delete: bool 
}
#[derive(Debug, Clone, GraphQLInputObject)]
pub struct NewProductPriceToUpdate { 
    pub data: ProductPriceInfoUpdate
}