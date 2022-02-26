use crate::schema::products;
use diesel::{RunQueryDsl, QueryDsl, query_builder::AsQuery, };
use crate::db::{establish_connection, DatabaseKind};
use diesel::result::Error as DbError;
use diesel::prelude::*;
#[derive(Queryable, Serialize, Deserialize, PartialEq)]
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
    pub fn get_product_info(id: &i32) -> Result<Product, DbError> { 
        use crate::schema::products;
        let conn = establish_connection();
        products::table.find(id).first(&conn)
    }
    pub fn delete_product(id: &i32) -> Result<(), DbError> { 
        use crate::schema::products::dsl;
        let conn = establish_connection();
        diesel::delete(
            dsl::products.find(&id))
            .execute(&conn)?;
        Ok(())
    }
    pub fn update_product(id: &i32, new_product: &NewProduct) -> Result<(), DbError> { 
        use crate::schema::products::dsl;
        let conn = establish_connection();
        diesel::update(
            dsl::products.find(id))
            .set(new_product)
            .execute(&conn)?;
        Ok(())

    }
    
    pub fn list_products() -> Self { 
        use crate::schema::products::dsl::*;
        use crate::db::DbPool;
        
        let conn = establish_connection();
        let res = products
            .limit(10)
            .load::<Product>(&conn)
            .expect("Error loading Products");

        ProductList(res)
    }
}

impl NewProduct { 
    pub fn create_product(&self) -> Result<Product, DbError> { 
        //  Insert new Product into database 
        //  Database Connection 
        use crate::schema::products;
        let conn = establish_connection();
        diesel::insert_into(products::table)
            .values(self)
            .get_result(&conn)
    }
}