use super::common::{footer::Footer, header::Header};
use wasm_bindgen::{prelude::Closure, JsCast};
use yew::{function_component, html, Callback};

#[function_component(Index)]
pub fn index() -> Html {
  

    html! {
        <>
            <Header />
            <>
            <div id="editor">
                {"首页"}
            </div>
            </>
            <Footer />
        </>
    }
}
