use yew::{prelude::*, function_component, html, Html};
use yew_router::prelude::*;
use crate::components::product::ProductProps;
use crate::hooks::cart_state::use_cart_context;
use crate::route::AppRoute;


#[function_component(Subtotal)]
pub fn subtotal(props: &ProductProps) -> Html {
    let subtotal = use_cart_context();
    let quantity = subtotal.basket.len();
    let price = subtotal.basket.iter()
        .fold(0.0, |acc, c| acc + c.price);


    let history = use_history().unwrap();
    let payment_page = Callback::once(move |_| history.push(AppRoute::PaymentPage));
    html! {
        <>
            <div >
                <div class="subtotal">
                    <>
                        <p>{
                            format!("Subtotal ({} items): ", quantity)}<strong>{format!("${}",price)}</strong>
                        </p>
                    </>
                    <small class="subtotal__gift">
                        <input type="checkbox"/>
                        {"This order contains a gift"}
                    </small>
                    
                    <button onclick={payment_page}>{"Proceed to Checkout"}</button>                
                </div>
            </div>
        </>
    }
}