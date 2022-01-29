pub mod pages;
pub mod route;
pub mod types;
pub mod service;
pub mod error;

use crate::pages::login::Login;
use log::Level;
use pages::index::Index;
use route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Login => html! (<Login />),
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::Index => html!(<Index />),
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(Level::Debug));
    yew::start_app::<App>();
}
