use crate::pages::common::{editor::Editor, header::Header};
use crate::pages::common::footer::Footer;
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
          <div class="row" style="margin:20px auto 0;">
            <div class="input-field col s8 offset-s2">
              <input id="title" type="text" />
              <label for="title">{"请用一句话描述你的问题"}</label>
            </div>
          </div>
          <div class="row">
            <div class="input-field col  s8 offset-s2">
              <Editor value={data.clone()} file_base_url={FILE_BASE_URL.to_string()} file_upload_url={FILE_UPLOAD_URL.to_string()} />
            </div>
          </div>
          <div class="row">
            <div class="col s2 offset-s8" style="text-align:right">
              <button class="btn waves-effect waves-light blue"  name="action">
                  {"发布问题"}<i class="material-icons right">{"send"}</i>
              </button>
            </div>
          </div>
          <Footer />
        </>
    }
}
