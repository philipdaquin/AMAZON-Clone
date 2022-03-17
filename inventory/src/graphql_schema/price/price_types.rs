use crate::schema::{prices, prices_products};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, Connection, PgConnection};
use diesel::{result::Error as DbError};
use itertools::Itertools;
use crate::models::product::Product;

#[derive(Queryable, Debug, Clone, Identifiable, Serialize, Deserialize, PartialEq)]
#[table_name = "prices"]
pub struct Price { 
    pub id: i32, 
    pub name: String,
    pub user_id: i32
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsChangeset, Insertable)]
#[table_name = "prices"]
pub struct NewPrice { 
    pub id: Option<i32>,
    pub name: Option<String>,
    pub user_id: Option<i32>
}
#[derive(Queryable, Debug, Clone, Associations, Identifiable, Serialize, Deserialize, PartialEq)]
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


#[derive(Serialize, Deserialize, Clone, Debug, Insertable, AsChangeset)]
#[table_name = "prices_products"]
pub struct NewPriceInfo { 
    pub id: Option<i32>, 
    pub price_id: i32,
    pub product_id: Option<i32>,
    pub user_id: Option<i32>,
    pub amount: Option<i32>
}
