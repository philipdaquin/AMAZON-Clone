use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::{
    header::Header, footer::Footer
};
use crate::route::home::Home;
use crate::route::{switch, AppRoute};
use super::state_provider::StateProvider;


pub struct App;

pub enum Msg {
}

#[derive(PartialEq, Properties, Clone, Default)]
pub struct Prop {
}
impl Component for App {
    type Message = Msg;
    type Properties = Prop;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool  {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
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