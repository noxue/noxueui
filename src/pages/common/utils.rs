use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    // 要显示的 html，请确保他是安全的，以防造成xss漏洞
    pub html: String,
    // 包裹这段html的标签名，默认为 div
    #[prop_or_default]
    pub tag_name: String,

    // html 显示在 id 指定的标签内部
    #[prop_or_default]
    pub append_id: String,
}

///
/// 第一种用法：
/// ```no_run
/// <SafeHtml html="html" />  => <div>html</div>
/// ```
/// 
/// 第二种用法：
/// ```no_run
/// <SafeHtml tag_name="p" html="html" />  => <p>html</p>
/// ```
/// 
/// 第三种用法：
/// ```no_run
/// <span id="test"></span>
/// <SafeHtml id="test" html="html" />  => <span>html</span>
/// ```
#[function_component(SafeHtml)]
pub fn safe_html(props: &Props) -> Html {

    // 如果没指定id就创建一个指定的标签来把html代码添加到里面
    // 有指定id 就把内容添加到 id 指定的标签中
    let div = if props.append_id.is_empty() {
        gloo_utils::document()
            .create_element(if props.tag_name.is_empty() {
                "div"
            } else {
                &props.tag_name
            })
            .unwrap()
    } else {
        gloo_utils::document()
            .get_element_by_id(&props.append_id)
            .unwrap()
    };

    div.set_inner_html(&props.html.clone());
    if props.append_id.is_empty() {
        return Html::VRef(div.into());
    }
    html!()
}
