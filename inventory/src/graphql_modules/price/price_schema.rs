use crate::graphql_modules::{MutationRoot, QueryRoot};
use juniper::{graphql_object, FieldResult, Object, };
use crate::graphql_modules::index::Context;
use super::price_types::*;


#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    pub fn prices(ctx: &Context) -> FieldResult<ListedPrice> { Price::list_prices(ctx) }
    pub fn find_price(ctx: &Context, price_id: i32) -> FieldResult<Price> { Price::find_price(ctx, price_id)}
}
#[graphql_object(context = Context)]
impl MutationRoot {
    pub fn create_price(ctx: &Context, new_price: NewPriceForm) -> FieldResult<Price> { Price::create_price(ctx, new_price)}
    pub fn update_price(ctx: &Context, price: NewPriceForm) -> FieldResult<Price> { Price::update_price(ctx, price)}
    pub fn destroy_price(ctx: &Context, price_id: i32) -> FieldResult<bool> { Price::destroy_price(ctx, price_id)}
}
