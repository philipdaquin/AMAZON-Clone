use yew::{prelude::*, function_component, html, Html};


#[derive(Clone, PartialEq, Properties)]
pub struct UserProviderProps { 
    #[prop_or_default]
    pub children: Children
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