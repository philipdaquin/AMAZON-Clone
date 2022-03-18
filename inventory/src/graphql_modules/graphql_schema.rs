use crate::graphql_modules::index::Context;
use juniper::{graphql_object, FieldResult};
use super::price::price_types::{Price};

pub struct QueryRoot;
pub struct MutationRoot;

#[graphql_object(context = Context)]
impl QueryRoot {
    //  Price
    pub fn prices(ctx: &Context) -> FieldResult<ListedPrice> { Price::list_prices(ctx) }
    pub fn find_price(ctx: &Context, price_id: i32) -> FieldResult<Price> { Price::find_price(ctx, price_id)}
    //  Sales
    pub fn list_sale(ctx: &Context, search: Option<String>, limit: i32) -> FieldResult<FullSale> { 
    } 
    pub fn show_sale() {}
    //  Product 
    pub fn list_products() {}
    pub fn show_products() {}
   
}
#[graphql_object(context = Context)]
impl MutationRoot {
    pub fn create_price(ctx: &Context, new_price: NewPriceForm) -> FieldResult<Price> { Price::create_price(ctx, new_price)}
    pub fn update_price(ctx: &Context, price: NewPriceForm) -> FieldResult<Price> { Price::update_price(ctx, price)}
    pub fn destroy_price(ctx: &Context, price_id: i32) -> FieldResult<bool> { Price::destroy_price(ctx, price_id)}
    pub fn create_product() {}
    pub fn update_product() {}
    pub fn destroy_prouct() {}
    pub fn create_sale() {}
    pub fn update_sale() {}
    pub fn approve_sale() {}
    pub fn pay_sale() {}
    pub fn destroy_sale() {}
    pub fn partially_pale() {}
}
