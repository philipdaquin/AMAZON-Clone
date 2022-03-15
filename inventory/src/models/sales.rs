use diesel::{result::Error as DbError, 
    PgConnection, BelongingToDsl, sql_types,
};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use crate::schema::{self, sales, sale_products};
use crate::types::{ProductColumns, PRODUCT_COLUMNS};
use juniper::{FieldResult, GraphQLObject, GraphQLInputObject};
use chrono::NaiveDate;
use crate::models::product_sales::{NewProductSaleInfo, ProductForSale};
use std::sync::Arc;

use super::product_sales::NewProductSale;

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq, GraphQLObject)]
#[table_name = "sales"]
#[graphql(description = "Sales Object")]
pub struct Sale { 
    pub id: i32, 
    pub user_id: i32,
    pub sale_date: NaiveDate,
    pub total: f64,
    pub bill_number: Option<String>
}
#[derive(Insertable, Deserialize, GraphQLInputObject,
    Serialize, AsChangeset, Debug, Clone, PartialEq)]
#[table_name = "sales"]
pub struct NewSale { 
    pub id: Option<i32>, 
    pub user_id: Option<i32>,
    pub sale_date: Option<NaiveDate>,
    pub total: Option<f64>,
}
#[derive(Debug, Clone, GraphQLObject)]
pub struct FullSale { 
    pub sale_info: Sale,
    pub sales_products: Vec<ProductForSale>
}
#[derive(Debug, Clone, GraphQLObject)]
pub struct FullNewSale { 
    pub sale: NewSale,
    pub sales_products: Vec<NewProductSale>
}
#[derive(Debug, Clone, GraphQLObject)]
pub struct ListSale { 
    pub data: Vec<FullSale>
}
pub struct Query;


type BoxedQuery<'a> = diesel::query_builder::BoxedSelectStatement<'a, 
        (
            sql_types::Integer,
            sql_types::Integer,
            sql_types::Date,
            sql_types::Float8,
            sql_types::Nullable<sql_types::Text>,
        ),
        schema::sales::table, diesel::pg::Pg
    >;

impl Sale { 
    pub fn search_records<'a>(select: Option<NewSale>) -> BoxedQuery<'a> {
        let mut query = 
    }
}