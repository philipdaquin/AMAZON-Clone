use crate::handlers::{product::{self, 
    index, create_newproduct, get_info, delete_product, update_product
}};
use actix_web::{web, App, HttpRequest, HttpServer, Responder};



pub async fn new_server(port: u32) -> std::io::Result<()> {
    HttpServer::new(move || {
        //  App Routes
        App::new()
            //  Everything under /product/
            .service(
                web::resource("/product")
                //  Returns a list of all products 
                .route(web::get().to(index))
                //  Creates a new product and returns its id    
                .route(web::post().to(create_newproduct))
            )
            .service(
                web::resource("/product/{id}")
                //  Returns information about the product with id
                .route(web::get().to(get_info))
                //  Marks the product with id as delete
                .route(web::delete().to(delete_product))
                //  Updates information about the product with id
                .route(web::put().to(update_product))
            )
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}



