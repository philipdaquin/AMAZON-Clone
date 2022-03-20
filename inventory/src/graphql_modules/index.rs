use std::sync::Arc;
use crate::authentication::logged_user::LoggedUser;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use juniper::http::{playground::playground_source, GraphQLRequest};
use serde::de::Error as SerdeError;
use crate::db::DbPool;
use juniper::{graphql_object, RootNode, EmptySubscription, FieldResult};
use super::mutation::MutationRoot;
use super::query::QueryRoot;

//  Context, an object shared by all the resolvers of a specific execution.
//  This gives us important contextual information like the currenly logged in user
//  or access to database 
#[derive(Clone)]
pub struct Context {
    pub user_id: i32,
    pub db_pool: Arc<DbPool>,
}
impl juniper::Context for Context {}
pub type SchemaGraphQL = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;
pub fn create_schema() -> SchemaGraphQL {
    SchemaGraphQL::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
pub fn create_context(user_id: i32, pool: DbPool) -> Context {
    Context { 
        user_id, 
        db_pool: Arc::new(pool)
    }
}
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
    data_body: web::Json<GraphQLRequest>,
    db_pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    
        let pool = (*db_pool).clone();
        let ctx = create_context(3, (*pool).to_owned());
        let res = data_body.execute(&st, &ctx).await;

    Ok(HttpResponse::Ok()
        .json(res))

}
