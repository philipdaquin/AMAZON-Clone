use diesel::sql_types;

use crate::schema::prices;
use crate::schema::prices_products;
use crate::schema::products;
use crate::schema::sale_products;
use crate::schema::sales;
//  We need to tell diesel we need only a few fields from the table,
// That's why we're going to need a constant that will tell Rust to bring all columns, 
// except TsVector, then we need a new modifications in the code to insert the filter required 

pub const PRICES: PricesType = (
    prices::id,
    prices::name, 
    prices::user_id
);
pub type PricesType = (
    prices::id,
    prices::name, 
    prices::user_id
);

pub const PRODUCT_COLUMNS: ProductColumns = (
    products::id,
    products::title,
    products::stock,
    products::rating,
    products::cost,
    products::description,
    products::user_id
);

pub type ProductColumns = (
    products::id,
    products::title,
    products::stock,
    products::rating,
    products::cost,
    products::description,
    products::user_id
);

pub const PRICE_PRODUCT: PriceProduct = (
    prices_products::id,
    prices_products::price_id,
    prices_products::product_id,
    prices_products::user_id,
    prices_products::amount
);

pub type PriceProduct = (
    prices_products::id,
    prices_products::price_id,
    prices_products::product_id,
    prices_products::user_id,
    prices_products::amount
);


pub const SALESCOLUMN: SalesColumn = (
    sales::dsl::id,
    sales::dsl::user_id,
    sales::dsl::sale_date,
    sales::dsl::total,
    sales::dsl::bill_number
);


pub type SalesColumn = (
    sales::dsl::id,
    sales::dsl::user_id,
    sales::dsl::sale_date,
    sales::dsl::total,
    sales::dsl::bill_number
);

pub const SALEPRODUCTS: SaleProducts = (
    sale_products::id,
    sale_products::product_id,
    sale_products::sale_id,
    sale_products::amount,
    sale_products::discount,
    sale_products::tax,
    sale_products::price,
    sale_products::total,
);

pub type SaleProducts = (
    sale_products::id,
    sale_products::product_id,
    sale_products::sale_id,
    sale_products::amount,
    sale_products::discount,
    sale_products::tax,
    sale_products::price,
    sale_products::total,
);


pub type SqlTypes = (
    sql_types::Integer,
    sql_types::Integer,
    sql_types::Date,
    sql_types::Float8,
    sql_types::Nullable<sql_types::Text>,
);