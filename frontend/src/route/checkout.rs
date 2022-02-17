use yew::{prelude::*, function_component, html, Html};
use crate::components::subtotal::Subtotal;
#[function_component(Checkout)]
pub fn checkout() -> Html {
    html! {
        <>
            <div class="checkout">
                <div class="checkout__left">
                    <img class="checkout__ad" src="
                    https://images-na.ssl-images-amazon.com/images/G/02/UK_CCMP/TM/OCC_Amazon1._CB423492668_.jpg
                    " alt=""/>
                    <div>
                        <h2 class="checkout__title">{"Your shopping basket"}</h2>
                        // Basket Items 
                    </div>
                </div>
                <div class="checkout__right">
                    <Subtotal/>
                </div>
            </div>
        </>
    }
}