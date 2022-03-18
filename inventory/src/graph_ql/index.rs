use std::sync::Arc;
use crate::authentication::logged_user::LoggedUser;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use juniper::http::{playground::playground_source, GraphQLRequest};
use crate::db::DbPool;
use juniper::{graphql_object, RootNode, EmptySubscription, FieldResult};
use super::graphql_schema::{MutationRoot, QueryRoot};

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
pub fn create_context(user_id: i32, db_pool: Arc<DbPool>) -> Context {
    Context { 
        user_id, 
        db_pool 
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

