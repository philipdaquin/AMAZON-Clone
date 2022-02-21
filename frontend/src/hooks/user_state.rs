use core::{fmt, ops::Deref};
use lazy_static::__Deref;
use yew_hooks::{use_async, use_mount};
use yew::{prelude::*, function_component, html, Html};
use yew_router::history::{AnyHistory, History};
use yew_router::hooks::use_history;
use crate::types::*;
use crate::route::AppRoute;
use crate::services::user_request::*;
use crate::services::{auth::*, user_request::*};
use crate::error::ServiceError;

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

impl fmt::Debug for UserContextHandler { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        f.debug_struct("Use User Context Hadnle")
            .field("value", &format!("{:?}", *self.inner))
            .finish()
    }
}

pub fn user_context_handler() -> UserContextHandler { 
    let inner = use_context::<UseStateHandle<UserInfo>>().unwrap();
    let history = use_history().unwrap();

    UserContextHandler { inner, history }
}

impl Deref for UserContextHandler {
    type Target = UserInfo;

    fn deref(&self) -> &Self::Target {
        &(*self.inner)
    }
}

impl Clone for UserContextHandler {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            history: self.history.clone(),
        }
    }
}

impl PartialEq for UserContextHandler {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}





#[function_component(UserContext)]
pub fn usercontext(UserProviderProps { children }: &UserProviderProps) -> Html {
    
    let user_context = use_state(UserInfo::default);
    let curr_user = use_async(async move {
        current_user().await
    });

    {
        let curr_user = curr_user.clone();
        use_mount(move || { 
            if get_token().is_some() { 
                curr_user.run()
            }
        });
    }

    {
        let user_context = user_context.clone();
        use_effect_with_deps(
            move |curr_user|  { 
            if let Some(user) = &curr_user.data { 
                user_context.set(user.user_info.clone());
            }
            if let Some(error) = &curr_user.error { 
                match error { 
                    ServiceError::UnAuthorised | ServiceError::Forbidden => set_token(None),  
                    _ => ()                       
                }
            }
            || ()

        }, curr_user)
    }

    html! {
        <>
            <ContextProvider<UseStateHandle<UserInfo>> context={user_context}>
                { for children.iter()}
            </ContextProvider<UseStateHandle<UserInfo>>>
        </>
    }
}

