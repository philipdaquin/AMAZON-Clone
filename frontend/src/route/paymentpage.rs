use yew::{prelude::*, function_component, html, Html};
use crate::components::product::{Product, ProductProps};
use yew_router::prelude::*;
use crate::{
    route::AppRoute,
    components::{
        subtotal::Subtotal,
        checkout_product::CheckOutProduct}, 
        hooks::{
            cart_state::use_cart_context, 
            user_state::user_context_handler}
    
};

#[function_component(PaymentPage)]
pub fn paymentpage() -> Html {
    
    let (cart_info, user_info) = (use_cart_context(), user_context_handler());
    
    let (user_email, cart_info) = ((*user_info).clone(), (*cart_info).clone());
    let item_list: Vec<Html> = cart_info
        .basket
        .iter()
        .map(|item: &ProductProps| return html! { 
            <>
                <CheckOutProduct
                    id={item.id.clone()}
                    title={item.title.clone()}
                    image={item.image.clone()}
                    rating={item.rating.clone()}
                    price={item.price.clone()} />
            </>
    }).collect();
    
    html! {
        <>
            <div class="payment">
                <div class="payment__container">
                    <h1>
                        {"Checkout("}<Link<AppRoute> to={AppRoute::Checkout}>{cart_info.basket.len()} {" Items"} </Link<AppRoute>>{")"}
                    </h1>
                    //  Payment Section - delivery Address
                    <div class="payment__section">
                        <div class="payment__title">
                            <h3>{"Delivery Address"}</h3>
                        </div>
                        <div class="payment__address">
                            <p>{user_email.email}</p>
                            <p>{"Santa Clara Valley"}</p>
                            <p>{"Los Angeles, CA"}</p>
                        </div>
                    </div>
                    //  Payment Section - Review Items 
                    <div class="payment__section">
                        <div class="payment__title"> 
                            <h3>{"Review items and delivery"}</h3>
                        </div>
                        <div class="payment__items">
                            {item_list}   
                        </div>
                    </div>
                    //  Payment Section - Payment Method 
                    <div class="payment__section">
                        <div class="payment__title">
                            <h3>{"Payment Method"}</h3>
                        </div>
                        <div class="payment__details">
                            // Strip Magic
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}