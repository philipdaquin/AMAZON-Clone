use crate::handlers::{product::{self, 
    index, create_newproduct, get_info, delete_product, update_product
}, payment::handle_stripe};
use actix_web::{web, App, HttpRequest, HttpServer, Responder, http::header};
use actix_cors::Cors;
use crate::db::{establish_connection, DatabaseKind};
use crate::graphql_modules::index::{graphql, playground};

pub async fn new_server(port: u32) -> std::io::Result<()> {
    
    HttpServer::new(move || {
        //  App Routes
        App::new()
            .data(establish_connection(DatabaseKind::ProductDb))
            //  Allowed Methods
            .wrap(Cors::default()
                .allowed_origin("http://localhost:8080")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
                .max_age(3600),
            )
            
            //  Everything under /product/
            .service(web::resource("/product")
                //  Returns a list of all products 
                .route(web::get().to(index))
                //  Creates a new product and returns its id    
                .route(web::post().to(create_newproduct))
            )
            
            //  ProductId {}
            .service(web::resource("/product/{id}")
                //  Returns information about the product with id
                .route(web::get().to(get_info))
                //  Marks the product with id as delete
                .route(web::delete().to(delete_product))
                //  Updates information about the product with id
                .route(web::put().to(update_product))
            )
            
            //  Payment Services
            .service(web::scope("/payment")
                .service(web::resource("/create")
                    .route(web::post().to(handle_stripe))
                )
            )
            
            //  GraphQl
            .service(web::resource("/graphql")
                .route(web::get().to(graphql))
                .route(web::post().to(graphql)),
            )
            .service(web::resource("/playground")
                .route(web::get().to(playground))
            )


    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}




