use crate::schema::prices;
use crate::schema::prices::dsl::*;
use crate::schema::products;
use crate::schema::products::dsl::*;

use super::product_types::*;
use crate::graphql_modules::price::price_types::*;
use crate::graphql_modules::index::Context;

use diesel::pg::Pg;
use diesel::{
    result::Error as DbError, 
    Connection, 
    PgConnection, 
    ExpressionMethods, 
    RunQueryDsl, 
    GroupedBy, 
    QueryDsl,
    BelongingToDsl, 
    BoolExpressionMethods
};

use diesel_full_text_search::{plainto_tsquery, TsRumExtensions, TsVectorExtensions};
use crate::types::*;
use juniper::FieldResult;

//  CRUD
impl Product  { 
    pub fn delete_product(ctx: &Context, product_id: i32) -> FieldResult<bool> { 
        use crate::schema::products::dsl::user_id;
        let conn = &ctx.db_pool.get()?;
        diesel::delete(
            products.filter(user_id.eq(ctx.user_id))
            .find(product_id))
            .execute(conn)?;
        Ok(true)
    }

    pub fn update_product(
        ctx: &Context, 
        curr_product: NewProduct, 
        new_prices: NewProductPriceToUpdate
    ) -> FieldResult<NewProductInfo> { 
        use crate::schema::products::user_id;
        let conn = &ctx.db_pool.get()?;
        //  Check if the product_id exists in the current database
        let product_id = curr_product.id.ok_or(DbError::QueryBuilderError("MIssing product id".into()))?;
        let new_product_info = NewProduct { 
            id: Some(product_id),
            ..curr_product
        };
        let updated_product = diesel::update(
            products.filter(user_id.eq(ctx.user_id))
            .find(product_id))
            .set(new_product_info)
            .returning(PRODUCT_COLUMNS)
            .get_result::<Product>(conn)?;
        let updated_full_product = ProductPriceInfoUpdate::update_product(new_prices, product_id, &ctx)?;
        
        Ok(NewProductInfo {
            product: updated_product,
            price_info: updated_full_product
        })
    }

    pub fn list_products(
        ctx: &Context, 
        rank: f64, 
        search_input: String, 
        limit: i32
    ) -> FieldResult<ListedProduct> { 
        use crate::schema::products::dsl::user_id;

        let conn = &ctx.db_pool.get()?;
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
            .limit(i64::from(limit))
            .load::<Product>(conn)?;
        
        let price_info = PriceInfo::belonging_to(&queried_products) 
            .inner_join(prices::table)    
            .select((PRICE_PRODUCT, PRICES))
            .load::<(PriceInfo, Price)>(conn)?
            .grouped_by(&queried_products);

        let full_product: Vec<(Product, Vec<(PriceInfo, Price)>)> = queried_products
            .into_iter()
            .zip(price_info)
            .collect();

        let vec_products = full_product
            .iter()
            .map(|info| { 
                let full_price_product = info.1
                    .iter()
                    .map(|product_price_info| ProductPriceInfo { 
                        price_info: product_price_info.0.clone(),
                        price: product_price_info.1.clone()
                    }).collect();
                NewProductInfo { 
                    product: info.0.clone(),
                    price_info: full_price_product
                }
            }).collect();
            
        Ok(ListedProduct { 
            data: vec_products
        })
    }
    pub fn get_product_info(product_id: &i32, user_id_: i32, ctx: &Context) -> FieldResult<NewProductInfo> { 
        use crate::schema::products::dsl::user_id;
        use crate::schema::products::table;

        let conn = &ctx.db_pool.get()?;

        //  Find the Product Selected
        let product: Product = products::table
            .select(PRODUCT_COLUMNS)
            .filter(user_id.eq(user_id_))
            .find(product_id)
            .first(conn)?;

        //  Find Records of products --> price info etc    
        let price_info: Vec<ProductPriceInfo> = PriceInfo::belonging_to(&product)
            .inner_join(prices::table)
            .load::<(PriceInfo, Price)>(conn)?
            .iter()
            .map(|product_info| ProductPriceInfo {
                price_info: product_info.0,
                price: product_info.1
            }).collect();

        let return_product = NewProductInfo {
            product,
            price_info,
        };
        
        Ok(return_product)
    }
    pub fn create_product(
        ctx: &Context, 
        new_product: NewProduct, 
        new_prices:  NewProductPriceToUpdate
    ) -> FieldResult<NewProductInfo> { 

        let conn = &ctx.db_pool.get()?;
        let new_product_info = NewProduct { 
            user_id: Some(ctx.user_id),
            ..new_product
        };
        //  Insert Product Info
        let product_created: Product  = diesel::insert_into(products::table)
            .values(new_product_info)
            .returning(PRODUCT_COLUMNS)
            .get_result::<Product>(conn)?;
        //  Update Price Info of Product 
        let full_product_info = 
        ProductPriceInfoUpdate::update_product(new_prices, product_created.id, ctx)?;
        
        let return_product = NewProductInfo {
            product: product_created,
            price_info: full_product_info
        };
        Ok(return_product)
    }
    
}
impl PartialEq<Product> for NewProduct { 
    fn eq(&self, other: &Product) -> bool {
        let new_product = self.clone();
        let product = other.clone();

        new_product.title == Some(product.title) &&
        new_product.stock == Some(product.stock) &&
        new_product.cost == product.cost &&
        new_product.description == product.description
        
    }
}