use diesel::{result::Error as DbError, 
    PgConnection, BelongingToDsl, sql_types,
};
use diesel::{RunQueryDsl, QueryDsl};
use crate::schema::{self, sales, sale_products};
use crate::types::{ProductColumns, PRODUCT_COLUMNS};
use juniper::FieldResult;
