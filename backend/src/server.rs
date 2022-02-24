


use actix_web::{web, App, HttpRequest, HttpServer, Responder};


pub async fn new_server(port: u32) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            //  Creates a new product and returns its id 
            .route("/product/", web::post().to())
            //  Returns information about the product with id
            .route("/product/{id}", web::get().to())
            //  Updates information about the product with id
            .route("/product/{id}", web::put().to())
            //  Marks the product with id as delete
            .route("/product/{id}", web::delete().to())
            //  Returns a list of all products 
            .route("/product/", web::get().to())


    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}



