use crate::pages::common::{header::Header,editor::Editor};
use yew::{function_component, html, use_state};

#[function_component(Ask)]
pub fn ask() -> Html {
    let data = use_state(String::default);
    html! {
          <>
            // <Header />
            <div style="padding:40px;">
            <Editor value={data.clone()} />
            </div>
            <div>{&*data}</div>
          </>
      }
}
