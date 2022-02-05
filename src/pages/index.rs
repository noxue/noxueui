use super::common::{footer::Footer, header::Header};
use yew::{function_component, html};

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <>
            <Header />
            <>
            {"首页"}
            </>
            <Footer />
        </>
    }
}
