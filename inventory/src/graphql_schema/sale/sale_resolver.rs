use crate::graphql::graphql_schema::Context;
use juniper::{graphql_object, FieldResult};
use crate::models::sales::*;
use crate::graphql_schema::resolver::{MutationRoot, QueryRoot};


#[graphql_object(context = Context)]
impl QueryRoot {
   pub fn list_sale(ctx: &Context, search: Option<String>, limit: i32) -> FieldResult<FullSale> { 

   } 
   pub fn show_sale() {}
   
   

}
#[graphql_object(context = Context)]
impl MutationRoot {
    pub fn create_sale() {}
    pub fn update_sale() {}
    pub fn approve_sale() {}
    pub fn pay_sale() {}
    pub fn destroy_sale() {}
    pub fn partially_pale() {}
}