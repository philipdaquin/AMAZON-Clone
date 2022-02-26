use actix_web::{HttpRequest, HttpResponse, web};
use crate::models::product::{ProductList, NewProduct};



pub fn index(req: HttpRequest) -> HttpResponse { 
    HttpResponse::Ok()
    .json(ProductList::list_products())
}

pub fn create(new_product: web::Json<NewProduct>) -> Result<HttpResponse, HttpResponse> { 
    new_product
        .create_product()
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
} 