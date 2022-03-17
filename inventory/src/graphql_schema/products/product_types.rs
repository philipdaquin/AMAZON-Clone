use crate::schema::products;
use diesel::{result::Error as DbError, PgConnection};
use diesel::{RunQueryDsl, QueryDsl};
use crate::types::{PRODUCT_COLUMNS, ProductColumns};

use crate::models::prices::PriceInfo;


use juniper::GraphQLObject;


#[derive(Queryable, Serialize, 
    Deserialize, PartialEq, Debug, Clone, Identifiable)]
pub struct Product { 
    pub id: i32, 
    pub title: String, 
    pub stock: f64,
    pub rating: Option<f64>, 
    pub price: Option<i32>,
    pub description: Option<String>,
    pub user_id: i32,
}


#[derive(Insertable, Deserialize, 
    Serialize, AsChangeset, Debug, Clone, PartialEq)]
#[table_name="products"]
pub struct NewProduct { 
    pub title: Option<String>,
    pub stock: Option<f64>,
    pub cost: Option<i32>,
    pub description: Option<String>,
    pub user_id: Option<i32>
}

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
pub struct NewProductInfo { 
    pub product: Product,
    pub price_info: todo!()
}

#[derive(Debug, Clone, GraphQLObject)]
pub struct ListedProduct { 
    pub data: Vec<NewProductInfo>
}
