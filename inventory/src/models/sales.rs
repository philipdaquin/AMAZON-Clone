use diesel::pg::Pg;
use diesel::{result::Error as DbError, 
    PgConnection, BelongingToDsl, sql_types,
};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use juniper::meta::Field;
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
use super::product::Product;

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
    pub fn update_sale(
        ctx: &Context,
        update_sale: NewSale,
        update_products_sale: NewSaleProducts
    ) -> FieldResult<FullSale> {
        use crate::schema::sales::dsl::{user_id};
        use crate::schema::sales::dsl::sales;
        
        let conn: &PgConnection = ctx.db_pool; 

        //  Only update if it has been created before 
        //  Find sale_id exists 
        let sale_id = update_sale.id
            .ok_or(ServerError::DatabaseRequestError("Sale Info is not found"))?;

        conn.transaction(|| { 
            let sale = diesel::update(sales)
                .filter(user_id.eq(ctx.user_id).find(sale_id))
                .set_form(&update_sale)
                .get_result::<Sale>(conn)?;
            
            let updated_products: Result<Vec<NewProductSaleInfo>, _> = update_products_sale
                .data
                .into_iter()
                .map(|full_sale_product| { 

                    let sales_products = diesel::update(schema::sale_products::table)
                        .set(full_sale_product.sale_info)
                        .get_result::<ProductSaleInfo>(conn);
                    if let Some(param_product) = full_sale_product.sale_info.product_id { 
                        let product = schema::products::table   
                            .select(PRODUCT_COLUMNS)
                            .find(param_product)
                            .first(conn);

                        Ok(NewProductSaleInfo { 
                            sale_info: sales_products?,
                            product: product?
                        })         
                    } else { 
                        Err(ServerError::InternalServerError)
                    }
                }).collect();
        })

    }
    pub fn list_sale(ctx: &Context, search: Option<NewSale>, limit: i32) -> FieldResult<ListSale> { 
        use crate::schema::sales;
        
        let conn: &PgConnection = &ctx.conn;
        let query = Sale::search_records(search);

        let query_sale: Vec<Sale> = query
            .filter(sales::dsl::user_id.eq(ctx.user_id))
            .limit(limit.into())
            .load::<Sale>(conn)?;
        
        let query_products = ProductSaleInfo::belonging_to(&query_sale)
            .inner_join(schema::products::table)
            .select((SALEPRODUCTS, PRODUCT_COLUMNS ))
            .load::<(ProductSaleInfo, Product)>(conn)?
            .grouped_by(&query_sale);
        
        let full_sale: Vec<(Sale, Vec<(ProductSaleInfo, Product)>)> = query_sale
            .into_iter()
            .zip(query_products)
            .collect();
    
        let vec_sale = full_sale    
            .iter()
            .map(|tuple| { 
                let full_sale_product = tuple.1
                    .iter()
                    .map(|product_sale| ProductForSale {
                        sale_info: product_sale.0.clone(),
                        product: product_sale.1.clone(),
                    }).collect();
                FullSale { 
                    sale_info: tuple.0.clone(),
                    sales_products: full_sale_product,
                }
            }).collect();
        
        Ok(ListSale { 
            data: vec_sale
        })
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
                    let new_sale_products = NewProductSale { 
                        sale_id: Some(sales.id),
                        ..new_sale.sale_info
                    };
                    
                    let sale_product = diesel::insert_into(sale_products::table)
                        .values(new_sale_products)
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
                    
                })
                .collect();
                
                Ok(FullSale { 
                    sale_info: sales,
                    sales_products: product_for_sale?,
                })
        })
    }
    pub fn show(ctx: &Context, sale_id: i32) -> FieldResult<FullSale> { 
        use crate::schema::sales::{table, user_id};

        let conn: &PgConnection = ctx.db_pool;
        let sale: Sale = table 
            .filter(user_id.eq(ctx.user_id))
            .find(sale_id)
            .first::<Sale>(conn)?;
        let product_sales = ProductSaleInfo::belonging_to(&sale)
            .inner_join(schema::products::table)
            .select((SALEPRODUCTS, PRODUCT_COLUMNS ))
            .load::<(ProductSaleInfo, Product)>(conn)?
            .iter()
            .map(|tuple| ProductForSale { 
                sale_info: tuple.0.clone(),
                product: tuple.1.clone()
            }).collect();

        Ok(FullSale { 
            sale_info: sale,
            sales_products: product_sales
        })
    }
}



















