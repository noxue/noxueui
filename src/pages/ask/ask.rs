use crate::pages::common::footer::Footer;
use crate::pages::common::utils::SafeHtml;
use crate::pages::common::{editor::Editor, header::Header};
use dotenv_codegen::dotenv;
use yew::{function_component, html, use_state};

const FILE_BASE_URL: &str = dotenv!("FILE_BASE_URL");
const FILE_UPLOAD_URL: &str = dotenv!("FILE_UPLOAD_URL");

// materialize-tags 插件需要的 css 和 js 文件
const TAGSJS: &str = include_str!("materialize-tags/materialize-tags.min.js");
const TYPEAHEADJS: &str = include_str!("materialize-tags/typeahead.bundle.min.js");
const TAGSCSS: &str = include_str!("materialize-tags/materialize-tags.min.css");

#[function_component(Ask)]
pub fn ask() -> Html {
    let data = use_state(String::default);
    html! {
        <>
          <Header />

          <div class="row" style="margin:20px auto 0;">
            <div class="input-field col s8 offset-s2">
              <input id="title" type="text" />
              <label for="title">{"在这里输入一句话描述你的问题"}</label>
            </div>
          </div>
          <div class="row">
            <div class="input-field col  s8 offset-s2">
              <Editor value={data.clone()} file_base_url={FILE_BASE_URL.to_string()} file_upload_url={FILE_UPLOAD_URL.to_string()} />
            </div>
          </div>
          <div class="row" style="margin:20px auto 0;">
            <div class="input-field col s8 offset-s2">
              <input id="tags" type="text" data-role="materialtags" />
              <label for="tags">{"在这里输入问题相关标签"}</label>
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

          
          <SafeHtml tag_name="script" html={TYPEAHEADJS} />
          <SafeHtml tag_name="script" html={TAGSJS} />
          <SafeHtml tag_name="style" html={TAGSCSS} />
          
        </>
    }
}
