use juniper::{graphql_object, FieldResult, Object};
use crate::graphql_modules::index::Context;
use crate::graphql_modules::price::price_types::*;
use super::sale::{sale_resolver::*, sale_types::*};
use super::price::{price_resolvers::*, price_types::*};
use super::products::{product_resolver::*, product_types::*};

pub struct QueryRoot;
#[graphql_object(context = Context)]
impl QueryRoot {
    
    pub fn test(context: &Context) -> FieldResult<String> {
        Ok("test123123123123".to_string())
    }
    //  Sales
    pub fn list_sale(ctx: &Context, search: Option<NewSale>, limit: i32) -> FieldResult<ListSale> {  Sale::list_sale(ctx, search, limit)} 

    //  Product
    pub fn list_products(ctx: &Context, rank: f64, search_input: String, limit: i32) -> FieldResult<ListedProduct> { Product::list_products(ctx, rank, search_input, limit)}
    pub fn get_product_info(product_id: i32, user_id_: i32, ctx: &Context) -> FieldResult<NewProductInfo> { Product::get_product_info(&product_id, user_id_, ctx)}

    //  Prices
    pub fn list_prices(ctx: &Context) -> FieldResult<ListedPrice> { Price::list_prices(ctx) }
    pub fn find_price(ctx: &Context, price_id: i32) -> FieldResult<Price> { Price::find_price(ctx, price_id)}
}
