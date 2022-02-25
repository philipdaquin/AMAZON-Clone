use crate::schema::products;
use diesel::{RunQueryDsl, QueryDsl, query_builder::AsQuery, };
use crate::db::{establish_connection, DatabaseKind};

#[derive(Queryable, Serialize, Deserialize, PartialEq)]
pub struct Product { 
    pub id: i32, 
    pub title: String, 
    pub stock: f64,
    pub rating: Option<f64>, 
    pub price: Option<i32>
}

#[derive(Insertable)]
#[table_name="products"]
pub struct NewProduct { 
    pub title: Option<String>,
    pub stock: Option<f64>,
    pub price: Option<i32>
}

#[derive(Serialize, Deserialize)]
pub struct ProductList(pub Vec<Product>);

impl ProductList  { 
    pub fn create_product() -> Self { todo!() }
    pub fn get_productinfo() -> Self { todo!() }
    pub fn delete_product() -> Self { todo!() }
    pub fn update_product() -> Self { todo!() }
    
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