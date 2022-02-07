use super::common::{footer::Footer, header::Header};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Blob, DragEvent, Event, FileReader};
use yew::{function_component, html, Callback};

#[function_component(Index)]
pub fn index() -> Html {
    let on_drop = Callback::from(move |e: DragEvent| {
        e.prevent_default();
        let v = e.data_transfer().unwrap();

        log::info!("{:#?}", e);
        let files = v.files().unwrap();
        for i in 0..files.length() {
            let reader = match FileReader::new() {
                Ok(v) => v,
                Err(e) => {
                    log::error!("{:?}", e);
                    continue;
                }
            };

            let file = match files.item(i) {
                Some(v) => v,
                None => {
                    log::error!("获取文件名失败");
                    continue;
                }
            };
            let name = file.name();

            if let Some(err) = reader.error() {
                log::error!("{:?}", err);
            }

            reader.read_as_data_url(&file);

            let onload = Closure::wrap(Box::new(move |event: Event| {
                let file_reader: FileReader = event.target().unwrap().dyn_into().unwrap();
                let psd = file_reader.result().unwrap();

                log::debug!("{:#?}", psd);
            }) as Box<dyn FnMut(_)>);

            reader.set_onload(Some(onload.as_ref().unchecked_ref()));
            onload.forget();
        }
    });
    let on_dragover = Callback::from(move |e: DragEvent| {
        e.prevent_default();
        log::info!("松开鼠标上传文件");
    });

    html! {
        <>
            <Header />
            <>
            <textarea style="height:500px;" ondrop={on_drop} ondragover={on_dragover}></textarea>
            </>
            <Footer />
        </>
    }
}
