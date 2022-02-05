use std::thread;
use std::time::Duration;

use crate::hooks::use_user_context;
use crate::route::Route;
use crate::types::auth::{LoginInfo, PhoneRegisterCode, RegisterInfo};

use super::common::header::Header;
use crate::service::auth::{login, phone_register_code, register};
use gloo::timers::callback::Interval;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, TargetCast};
use yew_hooks::use_async;
use yew_router::history::History;
use yew_router::hooks::use_history;

#[function_component(Register)]
pub fn register() -> Html {
    let timer = use_state(i32::default);

    let ctx = use_user_context();
    let history = use_history().unwrap();

    if ctx.is_authenticated() {
        history.push(Route::Index);
        return html!();
    }

    let reg_info = use_state(RegisterInfo::default);

    let oninput_username = {
        let reg_info = reg_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*reg_info).clone();
            info.username = input.value();

            // 修改注册类型为phone，表示手机注册
            info.r#type = "phone".to_string();

            reg_info.set(info);
        })
    };

    let oninput_password = {
        let reg_info = reg_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*reg_info).clone();
            info.password = input.value();
            reg_info.set(info);
        })
    };

    let oninput_confirm_password = {
        let reg_info = reg_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*reg_info).clone();
            info.password_confirm = input.value();
            reg_info.set(info);
        })
    };

    let oninput_code = {
        let reg_info = reg_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*reg_info).clone();
            info.code = input.value();
            reg_info.set(info);
        })
    };

    // 用户注册
    let user_register = {
        let reg_info = reg_info.clone();
        use_async(async move { register((*reg_info).clone()).await })
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
        user_register.clone(),
    );

    let on_submit = Callback::from(move |e: MouseEvent| {
        e.prevent_default(); /* Prevent event propagation */
        let user_register = user_register.clone();
        user_register.run();
    });

    // 获取验证码
    let get_phone_code = {
        let reg_info = reg_info.clone();
        use_async(async move {
            phone_register_code(PhoneRegisterCode {
                phone: (&*reg_info).username.clone(),
            })
            .await
        })
    };

    Interval::new(10, || log::debug!("Example of a standalone callback."));

    let tr = timer.clone();
    use_effect_with_deps(
        move |res| {
            if let Some(res) = &res.data {
                log::info!("{:?}", res.data);
                let mut t = *tr;

                t = 60;

                Interval::new(1000, move || {
                    if t <= 0 {
                        return;
                    }

                    t -= 1;
                    tr.set(t);
                    log::info!("计时器:{}", t);
                })
                .forget();
            }

            || ()
        },
        get_phone_code.clone(),
    );

    let tr = timer.clone();
    let on_get_phone_code = Callback::from(move |e: MouseEvent| {
        e.prevent_default(); /* Prevent event propagation */

        // 如果等待获取过程中，不重新获取验证码
        if *tr > 0 {
            return;
        }

        let reg_info = reg_info.clone();
        log::info!("获取验证码:{:?}", &*reg_info);

        let get_phone_code = get_phone_code.clone();
        get_phone_code.run();
    });

    html! {
        <>
            <div class="login-wrap  bg-primary">
            <Header />
            <div class="row">
                <div class="login-box register-box row col l4 offset-l4 m8 offset-m2 s12 offset-s0">
                    <h3 class="center">{"用户注册"}</h3>
                    <form class="col s12" >
                        <div class="row">
                            <div class="input-field col s12">
                                <input oninput={oninput_username} id="first_name" type="text" class="validate" />
                                <label for="first_name">{"手机号"}</label>
                            </div>
                        </div>
                        <div class="row">
                            <div class="input-field col s12">
                                <input id="password" type="password" oninput={oninput_password} class="validate" />
                                <label for="password">{"密码"}</label>
                            </div>
                        </div>
                        <div class="row">
                            <div class="input-field col s12">
                                <input id="password-confirm" type="password" oninput={oninput_confirm_password} class="validate" />
                                <label for="password-confirm">{"确认密码"}</label>
                            </div>
                        </div>

                        <div class="row">
                            <div class="input-field col s8">
                                <input id="code" oninput={oninput_code} type="text"
                                class="validate" />
                                <label for="code">{"验证码"}</label>
                            </div>
                            <div class="input-field col s4">
                                <div onclick={on_get_phone_code} class="btn waves-effect waves-light light-blue">{format!("{}获取验证码",&*timer)}</div>
                            </div>
                        </div>
                        <div class="login-submit-btn" onclick={on_submit}>{"注册"}</div>
                    </form>
                </div>
            </div>
            </div>
        </>
    }
}
