use yew::{prelude::*, function_component, html, Html};
use crate::state_provider::{StateContext, Action, use_cart_context};
// use crate::types::ProductType;
#[derive(Properties, PartialEq, Clone)] 
pub struct ProductProps {
    #[prop_or_default]
    pub id: i32,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub image: String,
    #[prop_or_default]
    pub rating: usize,
    #[prop_or_default]
    pub price: f32
}

#[function_component(Product)]
pub fn productcontainer(ProductProps { 
        id,
        title, 
        image, 
        rating, 
        price 
    }: &ProductProps) -> Html {
   

    let product_type = ProductProps { 
        id: id.clone(),
        title: title.clone(), 
        image: image.clone(), 
        rating: rating.clone(), 
        price: price.clone()
    };
    let cart_product = use_cart_context();
    let add_to_cart = { 
        let cart_product = cart_product.clone();
        Callback::from(move |e: MouseEvent| cart_product.dispatch(Action::AddToCart(product_type.clone())))
    };
    html! {
        <>
            <div class="product">
                <div class="product__info">
                    <p>{title}</p>
                    <p class="product__price">
                        <small>{"$ "}</small>
                        <strong>{price}</strong>
                    </p>
                    <div class="product__rating">
                        <p>{format!("{}", "⭐".repeat(*rating))}</p>
                    </div>
                </div>
                <img src={image.clone()} alt=""/>
                <button class="button" onclick={add_to_cart}>{"Add to Basket"}</button>    
            </div>
        </>
    }
}