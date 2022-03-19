use crate::graphql_modules::{MutationRoot, QueryRoot};
use juniper::{graphql_object, FieldResult, Object};
use crate::graphql_modules::index::Context;
use super::product_types::*;


#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    pub fn list_products() {}
    pub fn show_products() {}
}
#[graphql_object(context = Context)]
impl MutationRoot {
    pub fn create_product() {}
    pub fn update_product() {}
    pub fn destroy_prouct() {}
}
