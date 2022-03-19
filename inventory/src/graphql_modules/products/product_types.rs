use crate::schema::products;
use juniper::{GraphQLObject, GraphQLInputObject};
use serde::{Deserialize, Serialize}; 
use crate::graphql_modules::price::price_types::ProductPriceInfo;

#[derive(Identifiable, Queryable, Serialize, 
    Deserialize, Debug, Clone, PartialEq, GraphQLObject)]
#[table_name = "products"]
pub struct Product { 
    pub id: i32, 
    pub title: String, 
    pub stock: f64,
    pub rating: Option<f64>, 
    pub price: Option<i32>,
    pub description: Option<String>,
    pub user_id: i32,
}
#[derive(Insertable, AsChangeset,  Deserialize, Debug,
    Serialize, Clone, PartialEq, GraphQLInputObject)]
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
    pub price_info: Vec<ProductPriceInfo>
}
#[derive(Debug, Clone, GraphQLObject)]
pub struct ListedProduct { 
    pub data: Vec<NewProductInfo>
}
