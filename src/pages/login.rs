use crate::types::auth::{LoginInfo, LoginInfoWrapper};

use super::common::{footer::Footer, header::Header};
use crate::service::auth::login;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, Component, Properties, TargetCast};
use yew_hooks::use_async;
use yew_router::prelude::*;



#[function_component(Login)]
pub fn login() -> Html {
    let login_info = use_state(LoginInfo::default);
    let oninput_username = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.username = input.value();
            login_info.set(info);
            log::info!("{}", login_info.username);
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
        use_async(async move {
            let request = LoginInfoWrapper {
                user: (*login_info).clone(),
            };
            login(request).await
        })
    };

    use_effect_with_deps(
        move |user_login| {
            if let Some(user_info) = &user_login.data {
                log::info!("{:?}", user_info);
                // ctx.login(user_info.user.clone());
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
                    <h3 class="center">{"用户登录"}</h3>
                    <form class="col s12" >
                        <div class="row">
                            <div class="input-field col s12">
                            <input placeholder="请输入用户名"  oninput={oninput_username} id="first_name" type="text" class="validate" />
                            <label for="first_name">{"用户名"}</label>
                            </div>
                        </div>
                        <div class="row">
                            <div class="input-field col s12">
                            <input id="password" placeholder="请输入密码" type="password" oninput={oninput_password} class="validate" />
                            <label for="password">{"密码"}</label>
                            </div>
                        </div>
                        <div class="login-submit-btn" onclick={on_submit}>{"登录"}</div>
                    </form>
                </div>
            </div>
            </div>
        </>
    }
}
