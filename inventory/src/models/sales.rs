use diesel::{result::Error as DbError, 
    PgConnection, BelongingToDsl, sql_types,
};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use crate::schema::{self, sales, sale_products};
use crate::types::{ProductColumns, PRODUCT_COLUMNS};
use juniper::FieldResult;
use chrono::NaiveDate;
use crate::models::product_sales::NewProductSaleInfo;


#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "sales"]
pub struct Sale { 
    pub id: i32, 
    pub user_id: i32,
    pub sale_date: NaiveDate,
    pub total: f64,
    pub bill_number: Option<String>
}

#[derive(Insertable, Deserialize, Serialize, AsChangeset, Debug, Clone, PartialEq)]
#[table_name = "sales"]
pub struct NewSale { 
    pub id: Option<i32>, 
    pub user_id: Option<i32>,
    pub sale_date: Option<NaiveDate>,
    pub total: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct FullSale { 
    pub sale: Sale,
    pub sales_products: Vec<FullNewSales>
}

pub struct FullNewSales { 
    pub sale: NewSale,
    pub sales_products: Vec<NewProductSaleInfo>
}

