use crate::models::product::{Product, NewProduct};



pub struct ProductSaleInfo { 
    pub id: i32,
    pub product_id: i32,
    pub sale_id: i32, 
    pub amount: i32, 
    pub discount: i32, 
    pub tax: i32, 
    pub price: i32,
    pub total: i32
}
pub struct ProductForSale  {
    pub sale_info: ProductSaleInfo,
    pub product: Product
}

pub struct NewProductSale { 
    pub id: Option<i32>,
    pub product_id: Option<i32>,
    pub sale_id: Option<i32>, 
    pub amount: Option<i32>, 
    pub discount: Option<i32>, 
    pub tax: Option<i32>, 
    pub price: Option<i32>,
    pub total: Option<i32>,
}

pub struct NewProductSaleInfo  {
    pub sale_info: NewProductSale,
    pub product: NewProduct
}

pub struct NewSaleProducts { 
    pub data: Vec<NewProductSaleInfo>
}