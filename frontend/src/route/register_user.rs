use yew::{prelude::*, function_component, html, Html};
use yew_hooks::use_async;
use yew_router::prelude::*;
use super::AppRoute;
use crate::{
        
};

#[function_component(RegisterUser)]
pub fn register_user() -> Html {
    

    html! {
        <>
            <div class="login">
                <Link<AppRoute> to={AppRoute::Home}>
                    <img class="login__logo" 
                    src="https://upload.wikimedia.org/wikipedia/commons/thumb/a/a9/Amazon_logo.svg/1024px-Amazon_logo.svg.png" alt=""/>
                </Link<AppRoute>>
            
                <div class="login__container">
                    <h1>{"Create Account"}</h1>
                    <form action="">
                        <h5>{"Your name"}</h5>
                        <input 
                            type="text" 
                            placeholder="Name" 
                            value={} 
                            oninput={} />
                        <h5>{"Password"}</h5>
                        <input 
                            type="text" 
                            placeholder="Password" 
                            value={} 
                            oninput={}/>
                        <button 
                            type="submit" 
                            class="login__siginbutton" 
                            onsubmit={}
                        >{"Continue"}</button>
                    </form>
                    <p>
                        {"By creating an account, you agree that you have read and accepted our Conditions of Use and Privacy Notice."}
                    </p>
                  
                </div>
            </div>
        </>
    }
}