use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::{
    header::Header, footer::Footer, login::Login
};
use crate::route::home::Home;
use crate::route::{switch, AppRoute};
use super::state_provider::StateProvider;


pub struct App { 
    pub signed_in: bool
}

pub enum Msg {
}

#[derive(PartialEq, Properties, Clone, Default)]
pub struct Prop {
}
impl Component for App {
    type Message = Msg;
    type Properties = Prop;

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            signed_in: true
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool  {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.signed_in { 
            html! {
                <BrowserRouter>
                    <Login/>
                        // <Switch<AppRoute> render={Switch::render(switch)} />
                    <Footer/>
                </BrowserRouter>
             }
        }  else {
            
            html! {
                <StateProvider>
                    <BrowserRouter>
                        <Header/>
                             <Switch<AppRoute> render={Switch::render(switch)} />
                        <Footer/>
                    </BrowserRouter>
                </StateProvider>
            }
        }
    }
}