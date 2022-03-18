use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, Connection, PgConnection};
use diesel::{result::Error as DbError};
use itertools::Itertools;
use juniper::FieldResult;
use crate::graph_ql::index::Context;
use crate::types::PRICE_PRODUCT;

use super::price_types::{
    Price, ListedPrice, ProductPriceInfo, NewPriceForm,
    PriceInfo, FormPriceInfo, ProductPriceInfoUpdate, 
    NewProductPriceToUpdate
};

//  The purpose is to associate users to the products they have, they should not have access to another users
//   list of products. WE are also associating prices to the product (another child to parent relationship) 
impl ProductPriceInfoUpdate { 
    //  We will produce aprice_id, product_id, and user_id
    pub fn update_product(
        records: NewProductPriceToUpdate, 
        product_id: i32,
        user_id: i32, 
        ctx: &Context
    ) -> Result<Vec<PriceInfo>, DbError> { 
        use crate::schema::prices_products;

        let conn: &PgConnection = ctx.db_pool; 
        conn.transaction(|| { 
            let mut to_keep = Vec::new();
            for info in records { 
                if info.delete && info.price_info.id.is_some() { 
                    diesel::delete(prices_products::table
                        .filter(prices_products::user_id.eq(user_id))
                        .find(info.price_info.id.unwrap())
                    ).execute(conn)?;
                } else { 
                    to_keep.push(info)
                }
            }
            let price_product_info = to_keep.iter().map(|price_product_info| { 
                let new_price_info = FormPriceInfo { 
                    user_id: Some(user_id),
                    product_id: Some(product_id),
                    ..price_product_info.clone().price_info
                };

                diesel::insert_into(prices_products::table)
                    .values(&new_price_info)
                    .on_conflict((prices_products::price_id, prices_products::product_id))
                    .do_update()
                    .set(prices_products::amount.eq(new_price_info.amount))
                    .returning(PRICE_PRODUCT).get_result::<PriceInfo>(conn)
            }).fold_ok(vec![], 
                |mut acc, val| { 
                acc.push(val);
                acc
            })?;

            let mut full_price_info = Vec::new();

            price_product_info.iter().for_each(|info| { 
                todo!()
            });

            Ok(full_price_info)

        })
    }
}
//  Resolvers is a collection of functions that generate response for a GraphQL query. 
//  It acts as a GraphQL Query Handler 
impl Price { 
    pub fn prices(ctx: &Context) -> FieldResult<ListedPrice> {
        use crate::schema::prices::dsl::{*, user_id};
        let conn: &PgConnection = &ctx.db_pool;
        let listed_price = ListedPrice { 
            //  Load the prices for the current user in context
            data:  prices.filter(user_id.eq(ctx.user_id))
            .load::<Price>(conn)?
        };
        Ok(listed_price)
    }
    pub fn find_price(ctx: &Context, price_id: i32) -> FieldResult<Price> {

    }
    pub fn create_price() {}
    pub fn update_price() {}
    pub fn destroy_price() {}

}