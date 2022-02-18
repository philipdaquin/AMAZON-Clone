use yew::{prelude::*, function_component, html, Html};
use super::product::ProductProps;
use crate::state_provider::{use_cart_context, Action};

#[function_component(CheckOutProduct)]
pub fn checkout_list(ProductProps { id, title, image, rating, price }: &ProductProps) -> Html {
    let cart_context = use_cart_context();
    let remove_from_basket = { 
        let cart_context = cart_context.clone();
        let id = id.clone();
        Callback::from(move |_| cart_context.dispatch(Action::RemoveToCart(id)))
    };

    html! {
        <>
            <div class="checkout_product">
                <img src={image.clone()} alt="" class="checkoutproduct__image"/>    
            
                <div class="checkoutproduct__info">
                    <p class="checkooutproduct__title">{title.clone()}</p>
                    <p class="checkoutproduct__price">
                        <small>{"$"}</small>
                        <strong>{price.clone()}</strong>
                    </p>
                    <div class="checkoutproduct__rating">
                        <p>{format!("{}", "⭐".repeat(rating.clone()))}</p>
                    </div>
                    <button onclick={remove_from_basket}>{"Remove from Basket"}</button>
                </div>
            </div>
        </>
    }
}