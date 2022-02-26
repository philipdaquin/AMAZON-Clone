use crate::schema::products;
use diesel::{result::Error as DbError, PgConnection};
use diesel::{RunQueryDsl, QueryDsl};
#[derive(Queryable, Serialize, 
    Deserialize, PartialEq, Debug, Clone, Identifiable)]
pub struct Product { 
    pub id: i32, 
    pub title: String, 
    pub stock: f64,
    pub rating: Option<f64>, 
    pub price: Option<i32>
}
#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="products"]
pub struct NewProduct { 
    pub title: String,
    pub stock: f64,
    pub price: Option<i32>
}

#[derive(Serialize, Deserialize)]
pub struct ProductList(pub Vec<Product>);

impl ProductList  { 
    pub fn get_product_info(id: &i32, conn: &PgConnection) -> Result<Product, DbError> { 
        use crate::schema::products;
        products::table.find(id).first(conn)
    }
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
    pub fn list_products(conn: &PgConnection) -> Self { 
        use crate::schema::products::dsl::*;
        let res = products
            .limit(10)
            .load::<Product>(conn)
            .expect("Error loading Products");

        ProductList(res)
    }
}
impl NewProduct { 
    pub fn create_product(&self, conn: &PgConnection) -> Result<Product, DbError> { 
        diesel::insert_into(products::table)
            .values(self)
            .get_result(conn)
    }
}