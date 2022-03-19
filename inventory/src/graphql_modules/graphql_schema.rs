use crate::graphql_modules::index::Context;
use juniper::{graphql_object, FieldResult};
use super::price::price_types::Price;
use super::{MutationRoot, QueryRoot};

#[graphql_object(context = Context)]
impl QueryRoot {
   
   
}
#[graphql_object(context = Context)]
impl MutationRoot {
   
    pub fn pay_sale() {}
    pub fn destroy_sale() {}
    pub fn partially_pale() {}
}
