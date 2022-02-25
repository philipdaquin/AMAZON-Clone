use actix_web::{HttpRequest, HttpResponse};

use crate::models::product::ProductList;
pub fn index(req: HttpRequest) -> HttpResponse { 
    HttpResponse::Ok()
    .json(ProductList::list_products())
}