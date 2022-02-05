use yew::{function_component, html};
use yew_router::prelude::*;

use crate::{hooks::use_user_context, route::Route};

#[function_component(Header)]
pub fn header() -> Html {
    let ctx = use_user_context();

    html! {
      <>
        <nav class="bg-primary">
          <div class="nav-wrapper">
            <Link<Route> to={Route::Index} classes="brand-logo">{"不学网"}</Link<Route>>
            <ul id="nav-mobile" class="right hide-on-med-and-down">
              <li><Link<Route> to={
                if ctx.is_authenticated() {Route::Index} else {Route::Login}
              }>{
                if ctx.is_authenticated() {&*ctx.username} else {"登录"}
              }</Link<Route>></li>
              <li>
              <Link<Route> to={
                if ctx.is_authenticated() {Route::Logout} else {Route::Register}
              }>{
                if ctx.is_authenticated() {"退出"} else {"注册"}
              }</Link<Route>>
              </li>
            </ul>
          </div>
        </nav>
      </>
    }
}
