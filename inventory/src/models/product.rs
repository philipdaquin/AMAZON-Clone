use crate::schema::products;
use diesel::{result::Error as DbError, PgConnection};
use diesel::{RunQueryDsl, QueryDsl};


//  We need to tell diesel we need only a few fields from the table,
// That's why we're going to need a constant that will tell Rust to bring all columns, 
// except TsVector, then we need a new modifications in the code to insert the filter required 
const PRODUCT_COLUMNS: ProductColumns = (
    products::id,
    products::title,
    products::stock,
    products::rating,
    products::price,
    products::description
);

type ProductColumns = (
    products::id,
    products::title,
    products::stock,
    products::rating,
    products::price,
    products::description,
);

#[derive(Queryable, Serialize, 
    Deserialize, PartialEq, Debug, Clone, Identifiable)]
pub struct Product { 
    pub id: i32, 
    pub title: String, 
    pub stock: f64,
    pub rating: Option<f64>, 
    pub price: Option<i32>,
    pub description: Option<String>
}
#[derive(Insertable, Deserialize, AsChangeset, Debug, Clone, PartialEq)]
#[table_name="products"]
pub struct NewProduct { 
    pub title: String,
    pub stock: f64,
    pub price: Option<i32>,
    pub description: Option<String>
}
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
    pub fn list_products(conn: &PgConnection, search_input: &str) -> Self { 
        use crate::schema::{products::dsl::*, self};
        use diesel::pg::Pg;
        use diesel_full_text_search::{plainto_tsquery, TsVectorExtensions};

        let mut query = schema::products::table.into_boxed::<Pg>();
        
        if !search_input.is_empty() {
            query = query.filter(
                text_searchable_product_col.matches(plainto_tsquery(search_input))
            )
        } 
        
        let res = products
            .select(PRODUCT_COLUMNS)
            .limit(10)
            .load::<Product>(conn)
            .expect("Error loading Products");

        ProductList(res)
    }
}

impl Product { 
    pub fn get_product_info(id: &i32, conn: &PgConnection) -> Result<Product, DbError> { 
        use crate::schema::products;
        products::table.find(id).select(PRODUCT_COLUMNS).first(conn)
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