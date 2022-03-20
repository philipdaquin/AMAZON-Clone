use yew::{prelude::*, function_component, html, Html};
use crate::components::product::{Product, ProductProps};
use yew_router::prelude::*;
use yew_hooks::use_async;
use crate::{
    route::AppRoute,
    components::{
        subtotal::Subtotal,
        checkout_product::CheckOutProduct,
        button::Button,
        card_component::CardElement}, 
        hooks::{
            cart_state::use_cart_context, 
            user_state::user_context_handler},
            services::payment_services::*,
    
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
    let client_secret = use_state(|| true);

    // let error = use_state(|| None);
    // let disabled = use_stawte(|| true);
    let onsubmit = {
        Callback::from(move |e: FocusEvent| { 
            //  Call stripe stuff
            //  payload  = await stripe 
            //  Direct User to Stripe!
        })
    };
    let handle_change = {
        Callback::from(move |e: InputEvent | { 
            // Listen for chages in the card element 
            // and display any errors as the customer types their card details 

            //  payload = await stripe.confirm card payment (client_secret ) { payment_method}
            //  card elements getelement (cardelement)
                //  pyament_method { card: elements get element card_elemnet}
                
        })
        //  then paymentintent  => paymet intent = payment confirmration 
        //  setsucceeeded true 
        //  seterror null
        //  setprocessing false 
        //  history.replace(/orders page)
    };
    //  Use Stripe 
    //  Stripe elements 
    //   Generating client secret 
    {
        let cart_info = cart_info.clone();
        use_effect_with_deps(move |_| { 
        //   Generate that client secret from stripe
        // use_async( async move { 
        //     // fetch(todo!()).await
        //     //  let response = Request::get("get client secret leuy ")
        //  method: Post
        //  url paymets create total? = ...  
        //  Stripe expects the ttal in a currencies submitted 
        // }).await;
        //  client_secret.set(reponse.data.clientsecret)
        || ()
    }, cart_info.basket);}

    //  CardElement 
    let quantity = cart_info.basket.len();
    let price = cart_info.basket.iter()
        .fold(0.0, |acc, c| acc + c.price);
    //  Link to pyament page stripe 
    let state = use_async(async move { 
        fetch_repo("jetli/yew-hooks".to_string()).await }
    );

    let on_load = {
        if state.loading {
            html! { "Loading, wait a sec..." }
        } else {
            html! {}
        }
    };
    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            let state = state.clone();
            // You can trigger to run in callback or use_effect_with_deps.
            state.run();
        })
    };
    let if_err = {
        if let Some(error) = &state.error {
            match error {
                Error::DeserializeError => html! { "DeserializeError" },
                Error::RequestError => html! { "RequestError" },
            }
        } else {
            html! {}
        }
    };


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
                            <form action="" onsubmit={onsubmit}>
                                //<CardElement onchange={handle_change}/>   
                                <div class="payment__price__container">
                                    <>
                                        <p>
                                            <strong>{format!("Order Total ${}", price)}</strong>
                                        </p>
                                    </>
                                </div>   
                                <button {onclick} disabled={state.loading}>
                                    <span>
                                        <p>
                                            {on_load}
                                        </p>
                                        {"Buy Now"}
                                    </span>
                                </button>   
                                    {if_err}                          
                            </form>
                            // Stripe Magic
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}