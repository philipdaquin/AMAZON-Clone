use actix_web::{HttpRequest, HttpResponse, web, post};
use crate::models::product::{ProductList, NewProduct, Product};



pub async fn index(req: HttpRequest) -> HttpResponse { 
    HttpResponse::Ok()
    .json(ProductList::list_products())
}
pub async fn create_newproduct(
    new_product: web::Json<NewProduct>) -> Result<HttpResponse, HttpResponse> { 
    new_product
        .create_product()
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
} 
pub async fn get_info(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> { 
    ProductList::get_product_info(&id)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|err| HttpResponse::InternalServerError().json(err.to_string()))
} 

pub async fn delete_product(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> { 
    ProductList::delete_product(&id)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
pub async fn update_product(id: web::Path<i32>, new_product: web::Json<NewProduct>) -> Result<HttpResponse, HttpResponse> { 
    ProductList::update_product(&id, &new_product)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}