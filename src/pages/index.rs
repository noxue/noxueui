use super::common::{footer::Footer, header::Header};
use wasm_bindgen::{prelude::Closure, JsCast};
use yew::{function_component, html, Callback};

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <>
            <Header />
            <>
            <div class="pure-g">
                <div class="pure-u-1-12"></div>
                <div class="pure-u-5-6"><p>{"Thirds"}</p></div>
                <div class="pure-u-1-12"></div>
            </div>
            </>
            // <Footer />
        </>
    }
}
