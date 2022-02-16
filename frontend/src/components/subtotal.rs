use yew::{prelude::*, function_component, html, Html};
use crate::components::product::Props;

#[function_component(Subtotal)]
pub fn subtotal(props: &Props) -> Html {
    let quantity = use_state(|| 0);
    let subtotal = use_state(|| 0.0);
    let add_cart = { 
        let subtotal = subtotal.clone();
        let props = props.clone();
        Callback::from(move |e: f32| subtotal.set(*subtotal + props.price))
    };

    html! {
        <>
            <div >
                <div class="subtotal">
                    <>
                        <p>{
                            format!("Subtotal ({} items): ", *quantity)}<strong>{*subtotal}</strong>
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