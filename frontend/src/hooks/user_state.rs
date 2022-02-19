use yew::{prelude::*, function_component, html, Html};
use yew_router::history::{AnyHistory, History};
use crate::types::*;
use crate::route::AppRoute;
use crate::services::user_request::*;
#[derive(Clone, PartialEq, Properties)]
pub struct UserProviderProps { 
    #[prop_or_default]
    pub children: Children
}

pub struct UserContextHandler { 
    pub inner: UseStateHandle<UserInfo>,
    pub history: AnyHistory
}
impl UserContextHandler { 
    pub fn login(&self, value: UserInfo) { 
        set_token(Some(value.token.clone()));
        self.inner.set(value);
        self.history.push(AppRoute::Home)
    }
    pub fn logout(&self) { 
        set_token(None);
        self.inner.set(UserInfo::default());
        self.history.push(AppRoute::Home)
    }
}






#[function_component(UserContext)]
pub fn usercontext(UserProviderProps { children }: &UserProviderProps) -> Html {
    
    
    html! {
        <>
            <div>

            </div>
        </>
    }
}