use web_sys::HtmlInputElement;
use yew::{prelude::*, function_component, html, Html};
use yew_hooks::use_async;
use yew_router::prelude::*;
use super::AppRoute;
use crate::{
    hooks::user_state::*, 
    types::{RegisterInfoWrapper, RegisterInfo},
    services::auth,
};
#[function_component(RegisterUser)]
pub fn register_user() -> Html {
    let user_context = user_context_handler();
    let register_info = use_state(RegisterInfo::default);
    let user_register = { 
        let register_info = register_info.clone();
        use_async(async move { 
            let request = RegisterInfoWrapper { 
                user: (*register_info).clone()
            };
            auth::register_user(request).await
        })

    };
    {
        use_effect_with_deps(
            move |user_register| {
                if let Some(user_info) = &user_register.data {
                    user_context.login(user_info.user_info.clone());
                }
                || ()
            },
            user_register.clone(),
        );
    }

    let (username, password, email)
    = (register_info.username.clone(), register_info.password.clone(), register_info.email.clone());

    let set_username = { 
        let register_info = register_info.clone();
        Callback::from( move |e: InputEvent| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.username = input.value();
            register_info.set(info)
        })
    };

    let set_password = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.password = input.value();
            register_info.set(info)
        })
    };
    let set_email = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.email = input.value();
            register_info.set(info)
        })
    };
    let on_register_user = { 
        let user_register = user_register.clone();
        Callback::from(move |e: FocusEvent | { 
            e.prevent_default();
            let user_register = user_register.clone();
            user_register.run()
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
                    <h1>{"Create Account"}</h1>
                    <form action="">
                        <h5>{"Your name"}</h5>
                        <input 
                            type="text" 
                            placeholder="Name" 
                            value={username} 
                            oninput={set_username} />
                        
                        <h5>{"Email"}</h5>
                        <input 
                            type="email" 
                            placeholder="Email" 
                            value={email} 
                            oninput={set_email}/>
                        
                            <h5>{"Password"}</h5>
                        <input 
                            type="text" 
                            placeholder="Password" 
                            value={password} 
                            oninput={set_password}/>
                        <button 
                            type="submit" 
                            class="login__siginbutton" 
                            onsubmit={on_register_user}
                        >{"Continue"}</button>
                    </form>
                    <p>
                        {"By creating an account, 
                        you agree that you have read and accepted our Conditions of Use and Privacy Notice."}
                    </p>
                </div>
            </div>
        </>
    }
}