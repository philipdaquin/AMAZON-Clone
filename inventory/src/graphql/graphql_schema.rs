//  We Will use this some other time as soon as I understand how GraphQl Architeturer works
//   in the meantime, I will use Juniper which is another rust substitute
// use async_graphql::{
//     http::{playground_source, GraphQLPlaygroundConfig},
//     Data, EmptyMutation, Schema 
// };
// use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

// //  Provides Playground GraphQL IDE
// pub async fn index_playground() -> HttpResponse { 
//     HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(playground_source(
//             GraphQLPlaygroundConfig::new("/").subscription_endpoint("/")
//         )
//     )
// }
// pub async fn index(schema: web::Data<BooksSchema>, req: GraphQLRequest) -> GraphQLResponse {
//     schema.execute(req.into_inner()).await.into()
// }

use std::sync::Arc;

use juniper::{graphql_object, RootNode};
use juniper::{EmptySubscription, FieldResult};
use crate::db::DbPool;
use crate::models::sales::{NewSale, ListSale};
use crate::models::product_sales::{};
#[derive(Clone)]
pub struct Context {
    pub user_id: i32,
    pub db_pool: Arc<DbPool>,
}
impl juniper::Context for Context {}

pub struct QueryRoot;

#[graphql_object(context = Context)]
impl QueryRoot {
   

}

pub struct MutationRoot;

#[graphql_object(context = Context)]
impl MutationRoot {
    
}

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