use std::sync::Arc;
use crate::authentication::logged_user::LoggedUser;
use actix_web::http::header::HeaderMap;
use actix_web::http::Method;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use juniper::http::{playground::playground_source, GraphQLRequest};
use juniper::serde::ser::Error as SerdeError;
use crate::db::DbPool;
use crate::models::errors::GraphQLErrors;

use super::graphql_schema::{SchemaGraphQL, create_context};

//  Get request 
pub async fn playground() -> HttpResponse {
    let html = playground_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
// Post Request
pub async fn graphql(
    req: HttpRequest,
    st: web::Data<Arc<SchemaGraphQL>>,
    logged_user: LoggedUser,
    data_body: Option<web::Json<GraphQLRequest>>,
    db_pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {

    let user = web::block(move || { 
        let pool = db_pool.get().map_err(|e| serde_json::Error::column(e))?;
        let ctx = create_context(logged_user.id, pool);
        let res = data_body.execute(&st, &ctx);

        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    }).await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))

}

