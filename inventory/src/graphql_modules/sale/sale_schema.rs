// use crate::graphql_modules::{MutationRoot, QueryRoot};
// use juniper::{graphql_object, FieldResult, Object};
// use crate::graphql_modules::index::Context;
// use super::sale_types::*;


// #[juniper::graphql_object(context = Context)]
// impl QueryRoot {
//     pub fn list_sale(ctx: &Context, search: Option<String>, limit: i32) -> FieldResult<FullSale> { } 
//     pub fn show_sale() {}
// }
// #[graphql_object(context = Context)]
// impl MutationRoot {
//     pub fn create_sale() {}
//     pub fn update_sale() {}
//     pub fn approve_sale() {}
// }
