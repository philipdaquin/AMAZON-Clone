use juniper::{graphql_object, FieldResult, Object};
use crate::graphql_modules::index::Context;
use crate::graphql_modules::price::price_types::*;
use crate::graphql_modules::sale::sale_types::*;
use crate::graphql_modules::products::product_types::*;
pub struct MutationRoot;
#[graphql_object(context = Context)]
impl MutationRoot {
    //  Sales
    pub fn create_sale(ctx: &Context, new_sale: NewSale, new_products_sale: NewSaleProducts) -> FieldResult<FullSale> { Sale::create_sale(ctx, new_sale, new_products_sale)}
    pub fn update_sale(ctx: &Context, update_sale: NewSale, update_products_sale: NewSaleProducts) -> FieldResult<FullSale> { Sale::update_sale(ctx, update_sale, update_products_sale)}
    pub fn delete_sale(ctx: &Context, sale_id: i32) -> FieldResult<bool> { Sale::delete_sale(ctx, sale_id)}
    //  Price
    pub fn create_price(ctx: &Context, new_price: NewPriceForm) -> FieldResult<Price> { Price::create_price(ctx, new_price)}
    pub fn update_price(ctx: &Context, price: NewPriceForm) -> FieldResult<Price> { Price::update_price(ctx, price)}
    pub fn destroy_price(ctx: &Context, price_id: i32) -> FieldResult<bool> { Price::destroy_price(ctx, price_id)}
    //  Product 
    pub fn create_product(ctx: &Context, new_product: NewProduct, new_prices:  NewProductPriceToUpdate) -> FieldResult<NewProductInfo> { Product::create_product(ctx, new_product, new_prices) }
    pub fn delete_product(ctx: &Context, product_id: i32) -> FieldResult<bool> { Product::delete_product(ctx, product_id)} 
    pub fn update_product(ctx: &Context, curr_product: NewProduct, new_prices: NewProductPriceToUpdate) -> FieldResult<NewProductInfo> { Product::update_product(ctx, curr_product, new_prices)}

}
