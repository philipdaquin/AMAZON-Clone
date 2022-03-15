use diesel::{result::Error as DbError, 
    PgConnection, BelongingToDsl, sql_types,
};
use diesel::{RunQueryDsl, QueryDsl};
use crate::schema::{self, sales, sale_products};
use crate::types::{ProductColumns, PRODUCT_COLUMNS};
use juniper::FieldResult;
use chrono::NaiveTime;





pub struct Sales { 
    pub id: i32, 
    pub user_id: i32,
    pub sale_date: NaiveTime,
    pub total: f64,
    pub bill_number: Option<String>
}

pub struct NewSale { 
    pub id: Option<i32>, 
    pub user_id: Option<i32>,
    pub sale_date: Option<NaiveTime>,
    pub total: Option<f64>,
    pub bill_number: Option<String>
}

pub struct FullSale { 
    pub sale: Sales,
    pub sales_products: Vec<FullNewSales>
}

pub struct FullNewSales { 
    pub sale: NewSale,
    pub sales_products: Vec<FullNewSaleProduct>
}

