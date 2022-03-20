
use actix_web::HttpResponse;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, http::header};
use actix_cors::Cors;
use crate::db::{establish_connection, DatabaseKind};
use crate::graphql_modules::index::{graphql, playground, create_schema};



pub async fn new_server(port: u32) -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    //  Database connection pool
    let db_pool = establish_connection(DatabaseKind::ProductDb);
    let schema = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        //  App Routes
        App::new()
            .data(db_pool.clone())
            .data(schema.clone())
            .wrap(Logger::default())
            //  Allowed Methods
            .wrap(Cors::default()
                .allowed_origin("http://localhost:8080")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
                .max_age(3600),
            )
            //  GraphQl Services
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




