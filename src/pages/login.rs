use crate::hooks::use_user_context;
use crate::route::Route;
use crate::types::auth::LoginInfo;

use super::common::header::Header;
use crate::service::auth::login;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, TargetCast};
use yew_hooks::use_async;
use yew_router::history::History;
use yew_router::hooks::use_history;

#[function_component(Login)]
pub fn login() -> Html {
    let ctx = use_user_context();
    let history = use_history().unwrap();

    if ctx.is_authenticated() {
        history.push(Route::Index);
        return html!();
    }

    let login_info = use_state(LoginInfo::default);
    let oninput_username = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.credential = input.value();
            login_info.set(info);
            log::info!("{}", login_info.credential);
        })
    };

    let oninput_password = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.password = input.value();
            login_info.set(info);
            log::info!("{}", login_info.password);
        })
    };

    let user_login = {
        let login_info = login_info.clone();
        use_async(async move { login((*login_info).clone()).await })
    };

    use_effect_with_deps(
        move |res| {
            if let Some(user_info) = &res.data {
                log::info!("{:?}", user_info);
                ctx.login(user_info.data.clone());
                history.push(Route::Index);
            }
            || ()
        },
        user_login.clone(),
    );

    let on_submit = Callback::from(move |e: MouseEvent| {
        e.prevent_default(); /* Prevent event propagation */
        let user_login = user_login.clone();
        user_login.run();
    });

    html! {
        <>
            <div class="login-wrap bg-primary">
            <Header />
            <div class="row">
                <div class="login-box row col l4 offset-l4 m8 offset-m2 s12 offset-s0">
                    <h5 class="col s6 offset-s3 center">{"????????????"}</h5>
                    <form class="col s12" >
                        <div class="row">
                            <div class="input-field col s12">
                                <input placeholder="??????????????????"  oninput={oninput_username} id="first_name" type="text" class="validate" />
                                <label for="first_name">{"?????????"}</label>
                            </div>
                        </div>
                        <div class="row">
                            <div class="input-field col s12">
                                <input id="password" placeholder="???????????????" type="password" oninput={oninput_password} class="validate" />
                                <label for="password">{"??????"}</label>
                            </div>
                        </div>
                        <div class="row valign-wrapper">
                            <div class="col s4 offset-s8 right-align">
                                <a  href="/forget" style=" margin:auto;">{"???????????????"}</a>
                            </div>
                        </div>
                        <div class="login-submit-btn" onclick={on_submit}>{"??????"}</div>
                    </form>
                </div>

            </div>
            </div>
        </>
    }
}
