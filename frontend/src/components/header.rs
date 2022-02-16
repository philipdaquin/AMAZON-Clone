use yew_router::prelude::*;
use yew::{prelude::*, function_component, html, Html};
use crate::route::AppRoute;


#[function_component(Header)]
pub fn setup_header() -> Html {
    html! {
        <>
            <header class="header">
                    <Link<AppRoute> to={AppRoute::Home}>
                        <img class="header__logo" src="http://pngimg.com/uploads/amazon/amazon_PNG11.png" alt=""/>
                    </Link<AppRoute>>
                    <div class="header__search">
                        <input type="text" placeholder="I'm looking for.." class="header__search__input" />
                        <i class="bx bx-search-alt-2 header__search__icon" ></i>
                    </div>
                    
                    
                    <div class="header__nav">

                        //  Sign In Page
                        <div class="header__option">
                            <span class="header__option1">{"Hello Guest"}</span>                            
                            <Link<AppRoute> to={AppRoute::Signin}>
                                <span class="header__option2">{"Sign In"}</span>
                            </Link<AppRoute>>
                        </div>
                        // Return & Orders
                        <div class="header__option">
                            <span class="header__option1">{"Return"}</span>                            
                            <Link<AppRoute> to={AppRoute::Order}>
                                <span class="header__option2">{"& Orders"}</span>
                            </Link<AppRoute>>
                        </div>
                        //  Your Prime
                        <div class="header__option">
                            <span class="header__option1">{"Your"}</span>                            
                            <Link<AppRoute> to={AppRoute::Prime}>
                                <span class="header__option2">{"Prime"}</span>
                            </Link<AppRoute>>
                        </div>
                        //  Shopping bag
                        <div class="header__option__basket">
                            <Link<AppRoute> to={AppRoute::Checkout}>
                                <i class= "bx bxs-shopping-bag" ></i>
                                <span class="header__option2 header__basket__count">{"0"}</span>
                            </Link<AppRoute>>
                        </div>
                    </div>
            </header>
        </>
    }
}
