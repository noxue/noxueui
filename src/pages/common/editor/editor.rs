

use crate::pages::common::utils::SafeHtml;
use pulldown_cmark::{Options, Parser};
use web_sys::{HtmlInputElement, InputEvent};
use yew::{function_component, html, use_state, Callback, TargetCast};

#[function_component(Editor)]
pub fn editor() -> Html {
    let md_data = use_state(String::default);
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

    html!(
          <>
          // 代码高亮插件 js 和 css
          <script src="https://cdn.bootcdn.net/ajax/libs/highlight.js/11.4.0/highlight.min.js"></script>
          <link href="https://cdn.bootcdn.net/ajax/libs/highlight.js/11.4.0/styles/github.min.css" rel="stylesheet" />

          // css 样式
            <SafeHtml tag_name="style" html={include_str!("editor.css")} />
            // github-markdown.css
            <SafeHtml tag_name="style" html={include_str!("github-markdown.css")} />
            <div class="editor">
                <textarea id="editor-input" class="editor-input styled-scrollbars" oninput={on_editor_input}></textarea>
                <div class="editor-preview styled-scrollbars markdown-body" id="editor-preview">
                    <SafeHtml append_id="editor-preview" html={md_data} />
                </div>
            </div>

            // 让 textarea 支持 tab 键
            <SafeHtml tag_name="script" html={r#"
            document.getElementById('editor-input').addEventListener('keydown', function(e) {
                if (e.key == 'Tab') {
                  e.preventDefault();
                  var start = this.selectionStart;
                  var end = this.selectionEnd;
              
                  // set textarea value to: text before caret + tab + text after caret
                  this.value = this.value.substring(0, start) +
                    "\t" + this.value.substring(end);
              
                  // put caret at right position again
                  this.selectionStart =
                    this.selectionEnd = start + 1;
                }
              });
            "#} />

            // 每次松开按键，就执行高亮代码函数
            <SafeHtml tag_name="script" html={r#"
            document.getElementById('editor-input').addEventListener('keyup', function(e) {
                if(hljs){
                    hljs.highlightAll();
                }
              });
            "#} />
        </>
    )
}
