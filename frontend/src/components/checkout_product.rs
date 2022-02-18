use yew::{prelude::*, function_component, html, Html};
use super::product::Props;
#[function_component(CheckOutProduct)]
pub fn checkout_list(Props { id, title, image, rating, price }: &Props) -> Html {
    html! {
        <>
            <div class="checkout_product">
                <img src={image.clone()} alt="" class="checkoutproduct__image"/>    
            
                <div class="checkoutproduct__info">
                    <p class="checkooutproduct__title">{title.clone()}</p>
                    <p class="checkoutproduct__price">{price.clone()}
                        <small>{"$"}</small>
                        <strong>{price.clone()}</strong>
                    </p>
                    <div class="checkoutproduct__rating">
                        <p>{format!("{}", "⭐".repeat(rating.clone()))}</p>
                    </div>
                    <button>{"Remove from Basket"}</button>
                </div>
            </div>
        </>
    }
}