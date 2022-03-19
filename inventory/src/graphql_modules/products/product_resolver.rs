use crate::schema::products::dsl::{*, user_id};
use crate::schema::products;

use crate::schema::prices;
use crate::schema::prices::dsl::*;


use diesel::pg::Pg;
use diesel::{result::Error as DbError, PgConnection, RunQueryDsl, QueryDsl, BelongingToDsl};
use diesel_full_text_search::plainto_tsquery;
use crate::types::{PRODUCT_COLUMNS};
use crate::models::prices::PriceInfo;
use super::product_types::{Product, NewProduct, NewProductInfo, ListedProduct};
use juniper::FieldResult;
use crate::graphql_modules::index::Context;
//  CRUD
impl Product  { 
    pub fn delete_product(ctx: &Context, product_id: i32 ) -> FieldResult<bool> { 

        let conn: &PgConnection = &ctx.db_pool;
        diesel::delete(
            products.filter(user_id.eq(ctx.user_id))
            .find(product_id))
            .execute(conn)?;
        Ok(true)
    }

    pub fn update_product(
        product_id: &i32, 
        new_product: &NewProduct, 
        ctx: &Context 
    ) -> Result<(), DbError> { 
        let conn: &PgConnection = &ctx.db_pool;

        diesel::update(
            products.find(product_id))
            .set(new_product)
            .execute(conn)?;
        Ok(())
    }

    pub fn list_products(
        ctx: &Context, 
        rank: f64, 
        search_input: String, 
        limit: i32
    ) -> FieldResult<ListedProduct> { 
        let conn: &PgConnection = &ctx.db_pool;
        let mut query = products::table.into_boxed::<Pg>();

        if !search_input.is_empty() {
            query = query
                .filter(text_searchable_product_col.matches(plainto_tsquery(search_input)))
                .order((product_rank.desc(), 
                text_searchable_product_col.distance(plainto_tsquery(search_input))
            ));
        } else { 
            query = query.order(product_rank.desc());
        }
        let queried_products = query
            .select(PRODUCT_COLUMNS)
            .filter(user_id.eq(ctx.user_id).and(product_rank.le(rank)))
            .limit(10)
            .load::<Product>(conn)
            .expect("Error loading Products");
        
        let price_info = PriceInfo::belonging_to(&queried_products) 
            .select(PRODUCT_COLUMNS)    
            .load::<(PriceInfo, Product)>(conn)?
            .grouped_by(&queried_products);
        


    }
    pub fn get_product_info(
        product_id: &i32, 
        user_id_: i32,
        ctx: &Context
    ) -> Result<(Product, Vec<PriceInfo>), DbError> { 
        use crate::schema::{products::*, self};
        use diesel::{RunQueryDsl, BelongingToDsl, QueryDsl, ExpressionMethods};
        let conn: &PgConnection = &ctx.db_pool;
        
        let product: Product = schema::products::table
            .select(PRODUCT_COLUMNS)
            .filter(user_id.eq(user_id_))
            .find(product_id)
            .first(conn)?;

        let product_prices = PriceInfo::belonging_to(&product).load::<PriceInfo>(conn)?;
        Ok((product, product_prices))
    }
}

impl NewProduct { 
    pub fn create_product(&self, ctx: &Context) -> Result<Product, DbError> { 
        let conn: &PgConnection = &ctx.db_pool;

        diesel::insert_into(products::table)
            .values(self)
            .on_conflict_do_nothing()
            .returning(PRODUCT_COLUMNS)
            .get_result(conn)
    }
}

impl PartialEq<Product> for NewProduct { 
    fn eq(&self, other: &Product) -> bool {
        let new_product = self.clone();
        let product = other.clone();

        new_product.title == Some(product.title) &&
        new_product.stock == Some(product.stock) &&
        new_product.cost == product.price &&
        new_product.description == product.description
        
    }
}