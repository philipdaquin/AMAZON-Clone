use yew::{prelude::*, function_component, html, Html};
use crate::{
    components::{subtotal::Subtotal,
        checkout_product::CheckOutProduct,
    }, state_provider::use_cart_context};
use crate::types::ProductType;
#[function_component(Checkout)]
pub fn checkout() -> Html {

    let cart_context = use_cart_context();
    let item_list = cart_context
        .basket.iter()
        .map(|item| { 
        let ProductType { id, title, image, rating, price } = item.clone();
        
        return html! { 
            <>
                <CheckOutProduct
                    id={id}
                    title={title}
                    image={image}
                    rating={rating}
                    price={price}
                />
            </>
        }
    }); 
    // let ProductType { id, title, image, rating, price } = item.clone();

    // let item_list = cart_context.basket.iter().map(|item| return html! { 
    //         <>
    //             <CheckOutProduct
    //             id={item.id}
    //             title={item.title}
    //             image={item.image}
    //             rating={item.rating}
    //             price={item.price}
    //         />
    //     </>
    // });
  
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
                        // {item_list}
                    </div>
                </div>
                <div class="checkout__right">
                    <Subtotal/>
                </div>
            </div>
        </>
    }
}