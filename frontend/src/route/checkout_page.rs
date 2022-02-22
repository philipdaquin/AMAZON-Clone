use yew::{prelude::*, function_component, html, Html};
use crate::{
    components::{subtotal::Subtotal,
        checkout_product::CheckOutProduct,
    }, hooks::cart_state::use_cart_context};
// use crate::types::ProductType;
use crate::components::product::{Product, ProductProps};
#[function_component(Checkout)]
pub fn checkout() -> Html {

    let cart_context = use_cart_context();
    let item_list: Vec<Html> = cart_context
        .basket
        .iter()
        .map(|item: &ProductProps| return html! { 
            <>
                <CheckOutProduct
                id={item.id.clone()}
                title={item.title.clone()}
                image={item.image.clone()}
                rating={item.rating.clone()}
                price={item.price.clone()}
            />
        </>
    }).collect();


    html! {
        <>
            <div class="checkout">
                <div class="checkout__left">
                    <img class="checkout__ad" src="
                    https://images-na.ssl-images-amazon.com/images/G/02/UK_CCMP/TM/OCC_Amazon1._CB423492668_.jpg
                    " alt=""/>
                    <div>
                        <h2 class="checkout__title">{"Your shopping basket"}</h2>
                        {item_list}
                    </div>
                </div>
                <div class="checkout__right">
                    <Subtotal/>
                </div>
            </div>
        </>
    }
}