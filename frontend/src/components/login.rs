use yew::{prelude::*, function_component, html, Html};
use crate::route::AppRoute;
use yew_router::prelude::*;


#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <>
            <div class="login">
                <Link<AppRoute> to={AppRoute::Home}>
                    <img class="login__logo" 
                        src="https://upload.wikimedia.org/wikipedia/commons/thumb/a/a9/Amazon_logo.svg/1024px-Amazon_logo.svg.png" alt=""/>
                </Link<AppRoute>>
            </div>
        </>
    }
}