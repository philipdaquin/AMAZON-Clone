use crate::graphql_modules::{MutationRoot, QueryRoot};
use juniper::{graphql_object, FieldResult, Object};
use crate::graphql_modules::index::Context;
use crate::graphql_modules::price::price_types::*;
use super::sale::{sale_resolver::*, sale_types::*};
use super::price::{price_resolvers::*, price_types::*};
use super::products::{product_resolver::*, product_types::*};


#[juniper::graphql_object(context = Context)]
impl QueryRoot {

    pub fn list_sale(ctx: &Context, search: Option<String>, limit: i32) -> FieldResult<FullSale> {  } 
    pub fn show_sale(ctx: &Context, sale_id: i32) -> FieldResult<FullSale> {}

    pub fn prices(ctx: &Context) -> FieldResult<ListedPrice> { Price::list_prices(ctx) }
    pub fn find_price(ctx: &Context, price_id: i32) -> FieldResult<Price> { Price::find_price(ctx, price_id)}
}
