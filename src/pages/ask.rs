use crate::pages::common::{editor::Editor, header::Header};
use dotenv_codegen::dotenv;
use yew::{function_component, html, use_state};

const FILE_BASE_URL: &str = dotenv!("FILE_BASE_URL");
const FILE_UPLOAD_URL: &str = dotenv!("FILE_UPLOAD_URL");

#[function_component(Ask)]
pub fn ask() -> Html {
    let data = use_state(String::default);
    html! {
        <>
          <Header />
          <div style="padding:40px;">
            <Editor value={data.clone()} file_base_url={FILE_BASE_URL.to_string()} file_upload_url={FILE_UPLOAD_URL.to_string()} />
          </div>
        </>
    }
}
