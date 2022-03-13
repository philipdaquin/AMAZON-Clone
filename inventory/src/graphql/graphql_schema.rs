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
use crate::models::thermostat_status::*;

#[derive(Clone)]
pub struct Context {
    pub db_pool: Arc<DbPool>,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[graphql_object(context = Context)]
impl QueryRoot {
    #[graphql(description = "Query the current (latest) thermostat status")]
    fn thermostat_status(context: &Context) -> FieldResult<ThermostatStatus> {
        let connection = &context.db_pool.get()?;

        let result = ThermostatStatus::get_latest(connection)?;
        Ok(result)
    }

    #[graphql(description = "Query the thermostat status history")]
    fn thermostat_status_history(context: &Context) -> FieldResult<Vec<ThermostatStatus>> {
        let connection = &context.db_pool.get()?;

        let results = ThermostatStatus::get_history(connection)?;
        Ok(results)
    }
}

pub struct MutationRoot;

#[graphql_object(context = Context)]
impl MutationRoot {
    #[graphql(description = "Set the thermostat status")]
    fn set_thermostat_status(
        context: &Context,
        data: NewThermostatStatus,
    ) -> FieldResult<ThermostatStatus> {
        let connection = &context.db_pool.get()?;

        ThermostatStatus::insert(connection, data)?;

        let result = ThermostatStatus::get_latest(connection)?;
        Ok(result)
    }
}

pub type SchemaGraphQL = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> SchemaGraphQL {
    SchemaGraphQL::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}

pub fn create_context(db_pool: Arc<DbPool>) -> Context {
    Context { db_pool }
}