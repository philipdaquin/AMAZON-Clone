// use crate::schema::prices;
// use crate::schema::prices::dsl::*;
// use crate::schema::products;
// use crate::schema::products::dsl::*;
use crate::schema::sales::dsl::*;
use crate::schema::sales;

use crate::graphql_modules::products::product_types::*;
use crate::graphql_modules::price::price_types::*;
use crate::graphql_modules::index::Context;

use diesel::pg::Pg;
use diesel::{
    result::Error as DbError, 
    Connection, 
    PgConnection, 
    ExpressionMethods, 
    RunQueryDsl, 
    GroupedBy, 
    QueryDsl,
    BelongingToDsl, 
    BoolExpressionMethods
};

use diesel_full_text_search::{plainto_tsquery, TsRumExtensions, TsVectorExtensions};
use crate::types::*;
use juniper::FieldResult;

type BoxedQuery<'a> = BoxedSelectStatement<'a, SqlTypes, schema::sales::table, Pg>;
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

    pub fn delete_sale(ctx: &Context, sale_id: i32) -> FieldResult<bool> {
        let conn = &ctx.db_pool.get()?;
        let delete = diesel::delete(sales.filter(user_id.eq(ctx.user_id)).find(sale_id)
        ).execute(conn)?;
        Ok(delete == 1)
    }
    pub fn update_sale(
        ctx: &Context,
        update_sale: NewSale,
        update_products_sale: NewSaleProducts
    ) -> FieldResult<FullSale> {
        let conn = &ctx.db_pool.get()?;
        //  Only update if it has been created before 
        //  Find sale_id exists 
        let sale_id = update_sale.id.ok_or(DbError::QueryBuilderError("Missing Id".into()))?;

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

            Ok(FullSale { 
                sale_info: sale,
                sales_products: updated_products,
            })
        })
    }
    pub fn list_sale(ctx: &Context, search: Option<NewSale>, limit: i32) -> FieldResult<ListSale> { 
        
        let conn = &ctx.db_pool.get()?;
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
        
        let conn = &ctx.db_pool.get()?;
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
    pub fn show_product(ctx: &Context, sale_id: i32) -> FieldResult<FullSale> { 

        let conn = &ctx.db_pool.get()?;
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



















