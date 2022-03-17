use crate::graphql_schema::graphql_schema::Context;
use juniper::{graphql_object, FieldResult};
use crate::models::sales::*;
use crate::graphql_schema::{MutationRoot, QueryRoot};


#[graphql_object(context = Context)]
impl QueryRoot {
   
   

}
#[graphql_object(context = Context)]
impl MutationRoot {
   
}
