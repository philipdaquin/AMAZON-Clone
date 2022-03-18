
use crate::schema::products;
use diesel::{result::Error as DbError, PgConnection};
use diesel::{RunQueryDsl, QueryDsl};
use crate::types::{PRODUCT_COLUMNS, ProductColumns};
use crate::models::prices::PriceInfo;
use super::product_types::*;


#[derive(Serialize, Deserialize)]
pub struct ProductList(pub Vec<Product>);

impl ProductList  { 
    pub fn delete_product(id: &i32, conn: &PgConnection) -> Result<(), DbError> { 
        use crate::schema::products::dsl;
        diesel::delete(dsl::products.find(&id))
            .execute(conn)?;
        Ok(())
    }
    pub fn update_product(id: &i32, 
        new_product: &NewProduct,
        conn: &PgConnection,
    ) -> Result<(), DbError> { 
        use crate::schema::products::dsl;
        diesel::update(dsl::products.find(id))
            .set(new_product)
            .execute(conn)?;
        Ok(())
    }
    pub fn list_products(conn: &PgConnection, rank: f64, search_input: &str) -> Self { 
        use crate::schema::{products::dsl::*, self};
        use diesel::{pg::Pg, ExpressionMethods, RunQueryDsl, QueryDsl};
        use diesel_full_text_search::{plainto_tsquery, TsVectorExtensions, TsRumExtensions};

        let mut query = schema::products::table.into_boxed::<Pg>();
        if !search_input.is_empty() {
            query = query
                .filter(text_searchable_product_col.matches(plainto_tsquery(search_input)))
                .order((product_rank.desc(), 
                text_searchable_product_col.distance(plainto_tsquery(search_input))
            ));
        } else { 
            query = query.order(product_rank.desc());
        }


        
        let res = query
            .select(PRODUCT_COLUMNS)
            // .filter(product_rank.le(rank))
            .limit(10)
            .load::<Product>(conn)
            .expect("Error loading Products");

        ProductList(res)
    }
}

impl Product { 
    pub fn get_product_info(
        product_id: &i32, 
        user_id_: i32,
        conn: &PgConnection
    ) -> Result<(Product, Vec<PriceInfo>), DbError> { 
        use crate::schema::{products::*, self};
        use diesel::{RunQueryDsl, BelongingToDsl, QueryDsl, ExpressionMethods};
        
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
    pub fn create_product(&self, conn: &PgConnection) -> Result<Product, DbError> { 
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