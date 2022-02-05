use yew::{function_component, html};
use yew_hooks::use_mount;
use yew_router::{history::History, hooks::use_history};

use crate::{hooks::use_user_context, route::Route};

#[function_component(Logout)]
pub fn logout() -> Html {
    let ctx = use_user_context();

    let history = use_history().unwrap();

    use_mount(move || {
        if ctx.is_authenticated() {
            ctx.logout();
            history.push(Route::Login);
        } else {
            history.back();
        }
    });

    html!()
}
