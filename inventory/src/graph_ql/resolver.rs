use crate::graph_ql::graphql_schema::Context;
use juniper::{graphql_object, FieldResult};
use crate::models::sales::*;


use crate::models::sales::Sale;


pub struct QueryRoot;
pub struct MutationRoot;

#[graphql_object(context = Context)]
impl QueryRoot {
    // //  Price
    // pub fn list_price(ctx: &Context) {}
    // pub fn find_price() {}
    // //  Sales
    // pub fn list_sale(ctx: &Context, search: Option<String>, limit: i32) -> FieldResult<FullSale> { 
    // } 
    // pub fn show_sale() {}
    // //  Product 
    // pub fn list_products() {}
    // pub fn show_products() {}
   
}
#[graphql_object(context = Context)]
impl MutationRoot {
    // pub fn create_price() {}
    // pub fn update_price() {}
    // pub fn destroy_price() {}
    // pub fn create_product() {}
    // pub fn update_product() {}
    // pub fn destroy_prouct() {}
    // pub fn create_sale() {}
    // pub fn update_sale() {}
    // pub fn approve_sale() {}
    // pub fn pay_sale() {}
    // pub fn destroy_sale() {}
    // pub fn partially_pale() {}
}
