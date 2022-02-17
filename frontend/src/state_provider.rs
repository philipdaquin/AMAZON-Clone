use std::{rc::Rc, borrow::BorrowMut};
use crate::components::product::{Props};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Product { 
    id: i32,
    title: String,
    image: String,
    rating: usize,
    price: f32,
}

// #[derive(Debug, PartialEq, Clone)]
// pub struct CartProduct { 
//     pub product: Product,
//     pub quantity: i32
// }
// #[derive(Debug, PartialEq, Clone)]
// pub struct State {
//     pub products: Vec<Product>,
//     pub basket: Vec<CartProduct>,
// }

// impl Default for State { 
//     fn default() -> Self {
//         State { 
//             products: vec![], 
//             basket: vec![]
//         }
//     }
// }

pub struct InitialState { 
    pub cart_product: Vec<Product>
}

pub enum Action { 
    AddToCart
}
impl Reducible for InitialState {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let msg = match action { 
            Action::AddToCart => { 
                self.cart_product.clone().push( Product{..self.to_owned()})
            }
            // Action::AddToCart(product_id) => {
            //     let mut product = self
            //         .products
            //         .iter()
            //         .find(|product| product.id == product_id)
            //         .expect("Could Not Find the Product Id");
            //     let cart_product = self
            //         .basket
            //         .iter()
            //         .find(|bsk: && CartProduct| bsk.product.id == product_id);
            //     if let Some(basket) = cart_product { 
            //         let mut basket = basket.clone();
            //         basket.quantity +=1;
            //     } else { 
            //         self.basket.clone().push(CartProduct { 
            //             product: product.clone(),
            //             quantity: 1
            //         })
            //     }
            // }
        };
        
        InitialState { 
            cart_product: self.cart_product.clone()
            // products: self.products.clone(),
            // basket: self.basket.clone()
        }.into()
    }
}
pub type StateContext = UseReducerHandle<InitialState>;
#[derive(Properties, Debug, PartialEq)]
pub struct StateProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(StateProvider)]
pub fn state(props: &StateProviderProps) -> Html {
    let msg = use_reducer(|| Product { 
        id: , 
        title: todo!(), 
        image: todo!(), 
        rating: todo!(), 
        price: todo!() 
    }.to_vec());

    html! {
        <ContextProvider<StateContext> context={msg}>
             {props.children.clone()}
        </ContextProvider<StateContext>>
    }
}