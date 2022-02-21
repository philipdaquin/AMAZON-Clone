pub mod home;
pub mod checkout_page;
pub mod order;
pub mod prime;
pub mod signin;
pub mod register_user;
use crate::route::{ home::Home, checkout_page::Checkout, 
    signin::Signin, order::Order,  prime::Prime, register_user::RegisterUser 
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

    #[at("/register_user")] 
    Register,


}
pub fn switch(routes: &AppRoute) -> Html { 
    match routes { 
        AppRoute::Home => html! { <Home />},
        AppRoute::Checkout => html! { <Checkout />},
        AppRoute::Signin => html! { <Signin />},
        AppRoute::Order => html! { <Order />},
        AppRoute::Prime => html! { <Prime />},
        AppRoute::Register => html! { <RegisterUser />},

    }
}