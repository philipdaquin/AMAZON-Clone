use yew::{prelude::*, function_component, html, Html};
use yew_router::prelude::*;
use super::AppRoute;



#[function_component(Signin)]
pub fn signin() -> Html {

    



    html! {
        <>
            <div class="login">
                <Link<AppRoute> to={AppRoute::Home}>
                    <img class="login__logo" 
                    src="https://upload.wikimedia.org/wikipedia/commons/thumb/a/a9/Amazon_logo.svg/1024px-Amazon_logo.svg.png" alt=""/>
                </Link<AppRoute>>
               

                <div class="login__container">
                    <h1>{"Sign In"}</h1>
                    <form action="">
                        <h5>{"Email"}</h5>
                        <input type="text" placeholder="Email"/>

                        <h5>{"Password"}</h5>
                        <input type="text" placeholder="Password"/>

                        <button type="submit" class="login__siginbutton">{"Sign In"}</button>
                    </form>
                    <p>
                        {"By signing-in you agree to the AMAZON FAKE CLONE Conditions of Use & Sale. Please
                        see our Privacy Notice, our Cookies Notice and our Interest-Based Ads Notice."}
                    </p>

                    <button class="login__registration">{"Create your Amazon Account"}</button>
                </div>
            </div>
        </>
    }
}