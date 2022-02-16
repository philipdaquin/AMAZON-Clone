pub mod home;
pub mod checkout;
pub mod order;
pub mod prime;
pub mod signin;

use crate::route::{ home::Home, checkout::Checkout, 
    signin::Signin, order::Order,  prime::Prime,
};
use yew_router::prelude::*;
use yew::prelude::*;



#[derive(Debug, Routable, PartialEq, Clone)]
pub enum AppRoute { 
    #[at("/")] 
    Home,

    #[at("/checkout")] 
    Checkout,

    #[at("/signin")] 
    Signin,

    #[at("/order")] 
    Order,

    #[at("/prime")] 
    Prime,
}
pub fn switch(routes: &AppRoute) -> Html { 
    match routes { 
        AppRoute::Home => html! { <Home />},
        AppRoute::Checkout => html! { <Checkout />},
        AppRoute::Signin => html! { <Signin />},
        AppRoute::Order => html! { <Order />},
        AppRoute::Prime => html! { <Prime />},

    }
}