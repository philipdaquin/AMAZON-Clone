use crate::schema::{prices, prices_products};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, Connection, PgConnection};
use diesel::{result::Error as DbError};
use itertools::Itertools;

use crate::graphql_schema::graphql_schema::Context;
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
        



        
        conn.transaction(|| { 
            let mut to_keep = vec![];
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
            to_keep.iter().map(|price_product_info| { 
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
                    .returning((
                        prices_products::id, 
                        prices_products::price_id, 
                        prices_products::product_id, 
                        prices_products::user_id, 
                        prices_products::amount
                    )).get_result::<PriceInfo>(conn)
            }).fold_ok(vec![], |mut acc, val| { 
                acc.push(val);
                acc
            })


        })
    }
}
