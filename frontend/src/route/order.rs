use yew::prelude::*;

pub struct Order;

pub enum Msg {
}

impl Component for Order {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>{"this is the Order Page"}</div>
        }
    }
}