use web_sys::HtmlInputElement;
use yew::{prelude::*, function_component, html, Html};
use yew_router::prelude::*;
use yew_hooks::use_async;
use super::AppRoute;
use crate::hooks::user_state::{UserContextHandler, user_context_handler};
use crate::types::{LoginInfo, LoginInfoWrapper};
use crate::services::auth::*;



#[function_component(Signin)]
pub fn signin() -> Html {
    let user_context = user_context_handler();
    let login = use_state(LoginInfo::default);
    
    let user_login = {
        let login = login.clone();
        use_async(async move  { 
            let request = LoginInfoWrapper { 
                user: (*login).clone()
            };
            login_user(request).await
        })
    };

    use_effect_with_deps(
        move |user_login| { 
            if let Some(info) = &user_login.data { 
                user_context.login(info.user_info.clone());
            } || ()
        }, user_login.clone()
    );
    
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

    let on_sign_in_button = {
        let user_login = user_login.clone();
        Callback::from(move |e: FocusEvent| { 
            e.prevent_default();
            let user_login = user_login.clone();
            user_login.run()
            //  Login stuff
        })
    };

    let history = use_history().unwrap();
    let register_user = Callback::once(move |_| history.push(AppRoute::Register));

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
                        <input 
                            type="text" 
                            placeholder="Email" 
                            value={email} 
                            oninput={set_email} />
                        <h5>{"Password"}</h5>
                        <input 
                            type="text" 
                            placeholder="Password" 
                            value={password} 
                            oninput={set_password}/>
                        <button 
                            type="submit" 
                            class="login__siginbutton" 
                            onsubmit={on_sign_in_button}
                        >{"Sign In"}</button>
                    </form>
                    <p>
                        {"By signing-in you agree to the AMAZON FAKE CLONE Conditions of Use & Sale. Please
                        see our Privacy Notice, our Cookies Notice and our Interest-Based Ads Notice."}
                    </p>
                    //  Direct User to Sign Up Page +
                    <button 
                        class="login__registration" 
                        onclick={register_user}
                        >{"Create your Amazon Account"}</button>
                </div>
            </div>
        </>
    }
}