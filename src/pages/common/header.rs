use yew::{function_component, html};
use yew_router::prelude::*;

use crate::route::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
      <>
        <nav class="bg-primary">
          <div class="nav-wrapper">
            <Link<Route> to={Route::Index} classes="brand-logo">{"不学网"}</Link<Route>>
            <ul id="nav-mobile" class="right hide-on-med-and-down">
              <li><Link<Route> to={Route::Login}>{"登录"}</Link<Route>></li>
              <li><a href="collapsible.html">{"注册"}</a></li>
            </ul>
          </div>
        </nav>
      </>
    }
}
