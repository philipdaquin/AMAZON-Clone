use yew::{prelude::*, function_component, html, Html};
use crate::components::product::Props;
use crate::state_provider::use_cart_context;
#[function_component(Subtotal)]
pub fn subtotal(props: &Props) -> Html {
    let subtotal = use_cart_context();
    let quantity = subtotal.basket.len();
    let price = subtotal.basket.iter()
        .fold(0.0, |acc, c| acc + c.price);
    html! {
        <>
            <div >
                <div class="subtotal">
                    <>
                        <p>{
                            format!("Subtotal ({} items): ", quantity)}<strong>{format!("{}",price)}</strong>
                        </p>
                    </>
                    <small class="subtotal__gift">
                        <input type="checkbox"/>
                        {"This order contains a gift"}
                    </small>
                    
                    <button>{"Proceed to Checkout"}</button>                
                </div>
            </div>
        </>
    }
}