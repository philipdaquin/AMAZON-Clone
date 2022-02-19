use std::{rc::Rc, borrow::BorrowMut};
use wasm_bindgen::JsValue;
use yew::prelude::*;
// use super::types::{CartProduct, ProductType};
use gloo_console::{self as console, log};
use std::fmt::{Display, Formatter, Result};
use crate::components::product::ProductProps;

#[derive(PartialEq, Clone, Default)]
pub struct Cart {
    pub basket: Vec<ProductProps>
}

pub enum Action { 
    AddToCart(ProductProps),
    RemoveToCart(i32)
}
impl Reducible for Cart {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let basket: Vec<ProductProps> = match action { 
            Action::AddToCart(product) =>  { 
                let mut basket: Vec<ProductProps> = self.basket.clone();
                basket.push(product);
                basket
            },
            Action::RemoveToCart(product_id) => { 
                let mut basket = self.basket.clone();
                let product = basket
                    .iter_mut()
                    //  Find a matching Index
                    .position(|props| props.id == product_id);

                if let Some(product_id) = product { 
                    basket.remove(product_id);
                } else { 
                    console::log!("Product Not Found for {}", product_id);
                }
                basket
            },
        };

        Cart { 
            basket: basket,
        }.into()
    }
}

pub type CartContext = UseReducerHandle<Cart>;

#[derive(Properties, PartialEq)]
pub struct CartProviderProps {
    #[prop_or_default]
    pub children: Children,
}

pub fn use_cart_context() -> CartContext { 
    let msg = use_context::<CartContext>().expect("Use_cart_product didnt wanna work lol");
    msg
}

#[function_component(CartProvider)]
pub fn cartprovider(props: &CartProviderProps) -> Html {
    let msg = use_reducer(Cart::default);

    html! {
        <ContextProvider<CartContext> context={msg}>
        { for props.children.iter() }
        </ContextProvider<CartContext>>
    }
}

