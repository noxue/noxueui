use gloo::utils::document;
use yew::{function_component, html};
use yew_router::prelude::*;

use crate::{hooks::use_user_context, route::Route};

#[function_component(Header)]
pub fn header() -> Html {
    let ctx = use_user_context();
    document();
    html! {
      <>
        <nav class="bg-primary">
          <div class="nav-wrapper">
            <Link<Route> to={Route::Index} classes="brand-logo center">{"不学网"}</Link<Route>>
            <ul class="">
              <li><Link<Route> to={Route::Ask}>{"提问"}</Link<Route>></li>
            </ul>
            <ul class="right">
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