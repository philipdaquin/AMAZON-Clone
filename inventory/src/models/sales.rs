use diesel::pg::Pg;
use diesel::{result::Error as DbError, 
    PgConnection, BelongingToDsl, sql_types,
};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use crate::schema::sales::sale_date;
use crate::schema::{self, sales, sale_products};
use crate::types::{ProductColumns, PRODUCT_COLUMNS, SALEPRODUCTS, SALESCOLUMN};
use juniper::{FieldResult, GraphQLObject, GraphQLInputObject, graphql_object};
use chrono::NaiveDate;
use crate::models::product_sales::{NewProductSaleInfo, ProductForSale, ProductSaleInfo};
use std::sync::Arc;
use crate::graphql::graphql_schema::Context;
use crate::error::ServerError;
use super::product_sales::{NewProductSale, NewSaleProducts};

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
    pub bill_number: Option<String>

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
    pub fn search_records<'a>(search: Option<NewSale>) -> BoxedQuery<'a> {
        let mut query = sales::table.into_boxed::<Pg>();
        if let Some(sale) = search { 
            if let Some(sale_sale_date) = sale.sale_date { 
                query = query.filter(sale_date.eq(sale_sale_date))
            }
            if let Some(sale_bill_number) = sale.bill_number { 
                query = query.filter(sale_date.eq(sale_bill_number))
            }
        }
        query
    }

    pub fn delete_sale(
        ctx: &Context, 
        sale_id: i32
    ) -> FieldResult<bool> {
        use crate::schema::sales::{dsl::{user_id, sales}};

        let conn: &PgConnection = ctx.db_pool;
        let delete = diesel::delete(sales.filter(user_id.eq(ctx.user_id)).find(sale_id)
        ).execute(conn)?;
        Ok(delete == 1)
    }
    pub fn update_sale(ctx: &Context) {}
    pub fn list_sale(ctx: &Context, search: Option<NewSale>, limit: i32) -> FieldResult<ListSale> { 
        let conn: &PgConnection = &ctx.conn;
    }
    pub fn create_sale(
        ctx: &Context,
        new_sale: NewSale,
        new_products_sale: NewSaleProducts
    )  -> FieldResult<FullSale> {
        use crate::schema::sales::{table};
        use crate::schema::sale_products;


        let conn: &PgConnection = ctx.db_pool;
        let new_sale = NewSale { 
            user_id: Some(ctx.user_id),
            ..new_sale
        };
        conn.transaction(|| { 
            let sales: Sale = diesel::insert_into(table)
                .values(new_sale)
                .returning(SALESCOLUMN)
                .get_result::<Sale>(conn)?;

            let product_for_sale: Result<Vec<NewProductSaleInfo>, _> = new_products_sale
                .data
                .iter()
                .map(|new_sale| { 
                    let new_sale_producst = NewProductSale { 
                        sale_id: Some(sales.id),
                        ..new_sale.sale_info
                    };
                    
                    let sale_product = diesel::insert_into(sale_products::table)
                        .values(new_sale_producst)
                        .returning(SALEPRODUCTS)
                        .get_result::<ProductSaleInfo>(conn);
                        
                    if let Some(param_product) = new_sale.sale_info.product_id { 
                        let product = schema::products::table   
                            .select(PRODUCT_COLUMNS)
                            .find(param_product)
                            .first(conn);

                        Ok(NewProductSaleInfo { 
                            sale_info: sale_product?,
                            product: product?
                        })
                    } else { 
                        Err(ServerError::DatabaseRequestError)
                    }
                        
                    
                }).collect();
                
        })
    }
}



















