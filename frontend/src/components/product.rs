use yew::{prelude::*, function_component, html, Html};
use crate::state_provider::{StateContext, Action};
#[derive(Properties, PartialEq, Clone, Debug)] 
pub struct Props {
    #[prop_or_default]
    pub id: i32,
    
    #[prop_or_default]
    pub title: String,
    
    #[prop_or_default]
    pub image: String,
    
    #[prop_or_default]
    pub rating: usize,

    #[prop_or_default]
    pub price: f32
}

#[function_component(Product)]
pub fn productcontainer(props: &Props) -> Html {
    let Props {
        id,
        title, 
        image, 
        rating, 
        price
    } = props.clone();

    let cart_product = use_context::<StateContext>().expect("State Context - Add to Cart Error");
    let add_to_cart = { 
        let cart_product = cart_product.clone();
        Callback::from(move |e: MouseEvent| cart_product.dispatch(Action::AddToCart(id)))
    };

    html! {
        <>
            <div class="product">
                <div class="product__info">
                    <p>{title}</p>
                    <p class="product__price">
                        <small>{"$"}</small>
                        <strong>{price}</strong>
                    </p>
                    <div class="product__rating">
                        <p>{format!("{}", "⭐".repeat(rating))}</p>
                    </div>
                </div>
                <img src={image} alt=""/>
                <button class="button" onclick={add_to_cart}>{"Add to Basket"}</button>    
            </div>
        </>
    }
}