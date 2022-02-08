use crate::pages::common::{header::Header,editor::Editor};
use yew::{function_component, html};

#[function_component(Ask)]
pub fn ask() -> Html {
    html! {
          <>
            <Header />
            <Editor />
          </>
      }
}
