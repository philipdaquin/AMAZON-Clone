use yew::{prelude::*, function_component, html, Html};

#[function_component(Footer)]
pub fn setup_footer() -> Html {
    html! {
        <>
            <div> 
                <h3>{"This is the footer section"}</h3>
            </div>
        </>
    }
}