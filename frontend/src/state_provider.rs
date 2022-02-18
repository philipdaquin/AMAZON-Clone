use std::{rc::Rc, borrow::BorrowMut};
use wasm_bindgen::JsValue;
use yew::prelude::*;
use super::types::{CartProduct, ProductType};
use gloo_console::{self as console, log};
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Clone, Default)]
pub struct State {
    pub basket: Vec<ProductType>
}

pub enum Action { 
    AddToCart(ProductType)
}
impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action { 
            Action::AddToCart(product) =>  { 
               let mut basket: Vec<ProductType> = self.basket.clone();
               basket.push(product);
            }
        }

        State { 
            basket: Vec::new(),
        }.into()
    }
}

pub type StateContext = UseReducerHandle<State>;

#[derive(Properties, PartialEq)]
pub struct StateProviderProps {
    #[prop_or_default]
    pub children: Children,
}

pub fn use_cart_context() -> StateContext { 
    let msg = use_context::<StateContext>().expect("Use_cart_product didnt wanna work lol");
    msg
}

#[function_component(StateProvider)]
pub fn stateprovider(props: &StateProviderProps) -> Html {
    let msg = use_reducer(State::default);

    html! {
        <ContextProvider<StateContext> context={msg}>
            {props.children.clone()}
        </ContextProvider<StateContext>>
    }
}

