use yew::{prelude::*, function_component, html, Html};

#[derive(Properties, PartialEq, Clone)]
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
                <button class="button">{"Add to Basket"}</button>    
            </div>
        </>
    }
}