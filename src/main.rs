pub mod error;
pub mod hooks;
pub mod pages;
pub mod route;
pub mod service;
pub mod types;

use crate::pages::{
    ask::Ask, forget::Forget, login::Login, logout::Logout, provider::UserContext,
    register::Register,
};
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
        Route::Logout => html!(<Logout />),
        Route::Register => html!(<Register />),
        Route::Forget => html!(<Forget />),
        Route::Ask => html!(<Ask />),
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <UserContext>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </UserContext>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(Level::Debug));
    yew::start_app::<App>();
}
