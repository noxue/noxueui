use crate::pages::common::utils::SafeHtml;
use pulldown_cmark::{Options, Parser};
use web_sys::{HtmlInputElement, InputEvent};
use yew::{function_component, html, Callback, Properties, TargetCast, UseStateHandle};


const EDITORJS:&str = include_str!("editor.js");

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub value: UseStateHandle<String>,
    #[prop_or_default]
    pub file_base_url: String,
    #[prop_or_default]
    pub file_upload_url: String,
    #[prop_or("file".to_string())]
    pub file_name: String,
}

#[function_component(Editor)]
pub fn editor(props: &Props) -> Html {
    let md_data = props.value.clone();
    let on_editor_input = {
        let md_data = md_data.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            md_data.set(input.value());
        })
    };

    let md_data = &*md_data.clone();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    let parser = Parser::new_ext(md_data, options);

    let mut md_data = String::new();
    pulldown_cmark::html::push_html(&mut md_data, parser);

    // 读取 editor.js 文件，并替换其中的配置地址
    let editorjs = EDITORJS
        .replace("FILE_BASE_URL", &props.file_base_url)
        .replace("FILE_UPLOAD_URL", &props.file_upload_url)
        .replace("FILE_NAME", &props.file_name);

    html!(
          <>
          // 代码高亮插件 js 和 css
          <script src="https://cdn.bootcdn.net/ajax/libs/highlight.js/11.4.0/highlight.min.js"></script>
          <script src="https://unpkg.com/axios/dist/axios.min.js"></script>
          <link href="https://cdn.bootcdn.net/ajax/libs/highlight.js/11.4.0/styles/github.min.css" rel="stylesheet" />

            // css 样式
            <SafeHtml tag_name="style" html={include_str!("editor.css")} />
            // github-markdown.css
            <SafeHtml tag_name="style" html={include_str!("github-markdown.css")} />
            <div id="editor" class="editor write">
                <div class="tab-bar">
                  <div class="write">{"编辑"}</div>
                  <div class="preview">{"预览"}</div>
                </div>
                <div class="editor-body">
                  <div class="editor-input-box">
                    <textarea id="editor-input" class="editor-input styled-scrollbars" oninput={on_editor_input}></textarea>
                    <div class="upload-bar">{"点击此处上传文件,支持拖拽上传,或直接粘贴文件"}</div>
                    <input type="file"  id="ediotr-file-input" multiple={true} />
                  </div>
                  <div class="editor-preview styled-scrollbars markdown-body" id="editor-preview">
                      <SafeHtml append_id="editor-preview" html={if md_data.is_empty() {"无内容".to_string()} else {md_data}} />
                  </div>
                </div>
            </div>
            <SafeHtml tag_name="script" html={editorjs} />
        </>
    )
}
