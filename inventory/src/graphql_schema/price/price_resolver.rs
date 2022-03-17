use crate::graphql_schema::graphql_schema::Context;
use juniper::{graphql_object, FieldResult};
use crate::graphql_schema::resolver::{MutationRoot, QueryRoot};
use crate::models::prices::Price;





#[graphql_object(context = Context)]
impl QueryRoot {
    pub fn list_price(ctx: &Context) {}
    pub fn find_price() {}
   

}
#[graphql_object(context = Context)]
impl MutationRoot {
    pub fn create_price() {}
    pub fn update_price() {}
    pub fn destroy_price() {}
}