use crate::pages::common::utils::SafeHtml;
use pulldown_cmark::{Options, Parser};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::{function_component, html, Callback, Properties, TargetCast, UseStateHandle};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub value: UseStateHandle<String>,
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

    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    let parser = Parser::new_ext(md_data, options);

    // Write to String buffer.
    let mut md_data = String::new();
    pulldown_cmark::html::push_html(&mut md_data, parser);

    // 编辑器编号，区分多个编辑器
    let mut editor_id = 0;

    html!(
          <>
          // 代码高亮插件 js 和 css
          <script src="https://cdn.bootcdn.net/ajax/libs/highlight.js/11.4.0/highlight.min.js"></script>
          <link href="https://cdn.bootcdn.net/ajax/libs/highlight.js/11.4.0/styles/github.min.css" rel="stylesheet" />

            // css 样式
            <SafeHtml tag_name="style" html={include_str!("editor.css")} />
            // github-markdown.css
            <SafeHtml tag_name="style" html={include_str!("github-markdown.css")} />
            <div id={editor_id=editor_id+1;format!("editor-{}", editor_id)} class="editor write">
                <div class="tab-bar">
                  <div class="write">{"编辑"}</div>
                  <div class="preview">{"预览"}</div>
                </div>
                <div class="editor-body">
                  <div class="editor-input-box">
                    <textarea id="editor-input" class="editor-input styled-scrollbars" oninput={on_editor_input}></textarea>
                    <div class="upload-bar">{"上传文件,支持拖拽上传,直接粘贴,或直接选择"}</div>
                  </div>
                  <div class="editor-preview styled-scrollbars markdown-body" id="editor-preview">
                      <SafeHtml append_id="editor-preview" html={if md_data.is_empty() {"无内容".to_string()} else {md_data}} />
                  </div>
                </div>
            </div>
            <SafeHtml tag_name="script" html={include_str!("editor.js")} />
        </>
    )
}
