use crate::service::{auth::current, request::get_token};
use crate::types::Res;
use crate::types::auth::UserInfo;
use yew::{
    function_component, html, use_state, Children, ContextProvider, Html, Properties,
    UseStateHandle, use_effect_with_deps,
};
use yew_hooks::{use_async, use_mount};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(UserContext)]
pub fn user_context(props: &Props) -> Html {
    let user_ctx = use_state(|| UserInfo::default());

    let current_user = use_async(async move { current().await });

    {
        let current_user = current_user.clone();
        use_mount(move||{
            current_user.run();
        });
    }

    {
        let user_ctx = user_ctx.clone();
        use_effect_with_deps(move|v|{
            if let Some(res) = &v.data{
                let data = &res.data;
                log::info!("{:#?}", data);
                user_ctx.set(data.clone());
            }
            
            ||{}
        }, current_user);
    }

    

    html! {
        <ContextProvider<UseStateHandle<UserInfo>> context={user_ctx}>
        { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserInfo>>>
    }
}

