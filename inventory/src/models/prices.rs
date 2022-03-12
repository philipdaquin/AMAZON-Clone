use crate::schema::{prices, prices_products};
use diesel::{RunQueryDsl, QueryDsl};
use diesel::{result::Error as DbError, PgConnection};


#[derive(Queryable, Debug, Clone, Identifiable,
Serialize, Deserialize, PartialEq)]
#[table_name="prices_products"]
pub struct PriceInfo {
    pub id: i32, 
    pub price_id: i32,
    pub product_id: Option<i32>,
    pub user_id: i32,
    pub amount: Option<i32>
}