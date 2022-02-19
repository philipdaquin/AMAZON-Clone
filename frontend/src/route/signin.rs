use web_sys::HtmlInputElement;
use yew::{prelude::*, function_component, html, Html};
use yew_router::prelude::*;
use super::AppRoute;
use crate::types::*;


#[function_component(Signin)]
pub fn signin() -> Html {

    let login = use_state(LoginInfo::default);
    let (email, password) = (login.email.clone(), login.password.clone());
    //  Inputing Login Information
    let set_email = { 
        let login = login.clone();
        Callback::from(move |e: InputEvent| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login).clone();
            info.email = input.value();
            login.set(info)
        })
    };

    let set_password = {
        let login = login.clone();
        Callback::from(move |e: InputEvent| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login).clone();
            info.password = input.value();
            login.set(info)
        })
    };

    let sign_in = {
        Callback::from(move |e: FocusEvent| { 
            //  Prevent refreshing
            e.prevent_default();
            //  Login stuff
        })
    };
    let register_user = { 
        Callback::from(move |e: FocusEvent| { 
            e.prevent_default();
            // Do Something AWS register 
        })
    };



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
                        <input type="text" placeholder="Email" value={email} oninput={set_email} />
                        <h5>{"Password"}</h5>
                        <input type="text" placeholder="Password" value={password} oninput={set_password}/>
                        <button type="submit" class="login__siginbutton" onclick={sign_in}>{"Sign In"}</button>
                    </form>
                    <p>
                        {"By signing-in you agree to the AMAZON FAKE CLONE Conditions of Use & Sale. Please
                        see our Privacy Notice, our Cookies Notice and our Interest-Based Ads Notice."}
                    </p>
                    <button class="login__registration" onclick={register_user}>{"Create your Amazon Account"}</button>
                </div>
            </div>
        </>
    }
}