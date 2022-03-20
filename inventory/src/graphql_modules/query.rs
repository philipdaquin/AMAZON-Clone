use juniper::{graphql_object, FieldResult, Object};
use crate::graphql_modules::index::Context;
use crate::graphql_modules::price::price_types::*;
use super::sale::{sale_resolver::*, sale_types::*};
use super::price::{price_resolvers::*, price_types::*};
use super::products::{product_resolver::*, product_types::*};

pub struct QueryRoot;
#[graphql_object(context = Context)]
impl QueryRoot {
    //  Sales
    pub fn list_sale(ctx: &Context, search: Option<NewSale>, limit: i32) -> FieldResult<FullSale> {  Sale::list_sale(ctx, search, limit)} 

    //  Product
    pub fn show_product(ctx: &Context, sale_id: i32) -> FieldResult<FullSale> { }

    //  Prices
    pub fn list_prices(ctx: &Context) -> FieldResult<ListedPrice> { Price::list_prices(ctx) }
    pub fn find_price(ctx: &Context, price_id: i32) -> FieldResult<Price> { Price::find_price(ctx, price_id)}
}
