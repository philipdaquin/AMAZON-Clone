use yew::prelude::*;

pub struct Signin;

pub enum Msg {
}

impl Component for Signin {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>{"this is the Signin Page"}</div>

        }
    }
}