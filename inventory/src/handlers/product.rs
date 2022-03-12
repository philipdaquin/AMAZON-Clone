use actix_web::{HttpRequest, HttpResponse, web, post};
use crate::models::product::{ProductList, NewProduct, Product};
use crate::db::{DbPool, DbPooledConnection};
use crate::types::*;
use crate::authentication::logged_user::LoggedUser;

#[derive(Deserialize)]
pub struct ProductQuery { 
    pub text: String
}

#[derive(Deserialize)]
pub struct ProductPagination { 
    pub rank: f64
}

pub async fn index(
    req: HttpRequest, 
    db: web::Data<DbPool>,
    query: web::Query<ProductQuery>,
    pagination: web::Query<ProductPagination>
) -> Result<HttpResponse, HttpResponse> { 
    let db_pool = db_handler(db)?; 
    Ok(HttpResponse::Ok()
        .json(ProductList::list_products(&db_pool, pagination.rank, &query.text)))
}
pub async fn create_newproduct(
    new_product: web::Json<NewProduct>, 
    db: web::Data<DbPool>
) -> Result<HttpResponse, HttpResponse> { 
    
    let db_pool = db_handler(db)?; 
        new_product
        .create_product(&db_pool)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
} 
pub async fn get_info(product_id: web::Path<i32>, db: web::Data<DbPool>, user: LoggedUser) -> Result<HttpResponse, HttpResponse> { 
    let db_pool = db_handler(db)?; 
    
    Product::get_product_info(&product_id, user.id, &db_pool)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|err| HttpResponse::InternalServerError().json(err.to_string()))
} 

pub async fn delete_product(id: web::Path<i32>, db: web::Data<DbPool>) -> Result<HttpResponse, HttpResponse> { 
    let db_pool = db_handler(db)?; 
    
    ProductList::delete_product(&id, &db_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
pub async fn update_product(
    id: web::Path<i32>, 
    new_product: web::Json<NewProduct>,
    db: web::Data<DbPool>
) -> Result<HttpResponse, HttpResponse> { 
    let db_pool = db_handler(db)?; 
    
    ProductList::update_product(&id, &new_product, &db_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn db_handler(pool: web::Data<DbPool>) -> Result<DbPooledConnection, HttpResponse> { 
    pool
    .get()
    .map_err(|error| 
        HttpResponse::InternalServerError().json(error.to_string()))
}


