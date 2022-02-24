use crate::schema::products;


#[derive(Queryable)]
pub struct Product { 
    pub id: i32, 
    pub title: String, 
    pub stock: f64,
    pub rating: i32, 
    pub price: Option<i32>
}

#[derive(Insertable)]
#[table_name="products"]
pub struct NewProduct { 
    pub title: Option<String>,
    pub stock: Option<f64>,
    pub price: Option<i32>
}