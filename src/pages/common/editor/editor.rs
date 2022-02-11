use crate::pages::common::utils::SafeHtml;
use pulldown_cmark::{Options, Parser};
use web_sys::{HtmlInputElement, InputEvent};
use yew::{function_component, html, Callback, TargetCast, Properties, UseStateHandle};

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

    html!(
          <>
          // 代码高亮插件 js 和 css
          <script src="https://cdn.bootcdn.net/ajax/libs/highlight.js/11.4.0/highlight.min.js"></script>
          <link href="https://cdn.bootcdn.net/ajax/libs/highlight.js/11.4.0/styles/github.min.css" rel="stylesheet" />

            // css 样式
            <SafeHtml tag_name="style" html={include_str!("editor.css")} />
            // github-markdown.css
            <SafeHtml tag_name="style" html={include_str!("github-markdown.css")} />
            <div class="editor write">
                <div class="tab-bar">
                  <div class="write">{"编辑"}</div>
                  <div class="preview">{"预览"}</div>
                </div>
                <div class="editor-body">
                  <textarea id="editor-input" class="editor-input styled-scrollbars" oninput={on_editor_input}></textarea>
                  <div class="editor-preview styled-scrollbars markdown-body" id="editor-preview">
                      <SafeHtml append_id="editor-preview" html={if md_data.is_empty() {"无内容".to_string()} else {md_data}} />
                  </div>
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

            // 切换
            <SafeHtml tag_name="script" html={r#"
              let editor = document.getElementsByClassName('editor')[0];

              // 点击预览
              document.querySelector('.editor .preview').addEventListener('click', function(e) {
                
                // 点击预览就高亮显示代码
                if(hljs){
                    hljs.highlightAll();
                }

                editor.classList.remove('write');
                editor.classList.add('preview');
              });
              
              // 点击编辑
              document.querySelector('.editor .write').addEventListener('click', function(e) {
                console.log("click preview")
                editor.classList.remove('preview');
                editor.classList.add('write');
              });
            "#} />
        </>
    )
}
