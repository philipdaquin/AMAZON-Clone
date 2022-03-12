use crate::schema::{prices, prices_products};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, Connection, PgConnection};
use diesel::{result::Error as DbError};
use itertools::Itertools;
use crate::models::product::Product;

#[derive(Queryable, Debug, Clone, Identifiable, Serialize, Deserialize, PartialEq)]
#[table_name = "prices"]
pub struct Price { 
    pub id: i32, 
    pub name: String,
    pub user_id: i32
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsChangeset, Insertable)]
#[table_name = "prices"]
pub struct NewPrice { 
    pub id: Option<i32>,
    pub name: Option<String>,
    pub user_id: Option<i32>
}
#[derive(Queryable, Debug, Clone, Associations, Identifiable, Serialize, Deserialize, PartialEq)]
#[belongs_to(Price)]
#[belongs_to(Product)]
#[table_name="prices_products"]
pub struct PriceInfo {
    pub id: i32, 
    pub price_id: i32,
    pub product_id: i32,
    pub user_id: i32,
    pub amount: Option<i32>
}

 
#[derive(Serialize, Deserialize, Clone, Debug, Insertable, AsChangeset)]
#[table_name = "prices_products"]
pub struct NewPriceInfo { 
    pub id: Option<i32>, 
    pub price_id: i32,
    pub product_id: Option<i32>,
    pub user_id: Option<i32>,
    pub amount: Option<i32>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PriceProductUpdate { 
    pub price_info: NewPriceInfo,
    pub delete: bool,
}
//  The purpose is to associate users to the products they have, they should not have access to another users
//   list of products. WE are also associating prices to the product (another child to parent relationship) 
impl PriceProductUpdate { 
    //  We will produce aprice_id, product_id, and user_id
    pub fn update_product(
        records: Vec<Self>, 
        product_id: i32,
        user_id: i32, 
        conn: &PgConnection
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
                let new_price_info = NewPriceInfo { 
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