use yew::prelude::*;

pub struct Prime;

pub enum Msg {
}

impl Component for Prime {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>{"this is the Prime Page"}</div>

        }
    }
}