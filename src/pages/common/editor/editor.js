//在rust代码中会被自动替换成真实地址
const file_upload_url = "FILE_UPLOAD_URL";
const file_base_url = "FILE_BASE_URL";


// 让 textarea 支持 tab 键
document.getElementById('editor-input').addEventListener('keydown', function (e) {
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

// 切换 编辑和预览
let editor = document.querySelector('.editor');


// 配置了上传文件的地址才支持上传文件
if (file_upload_url != "" && file_base_url != "") {
    // 绑定拖拽上事件
    bindDrag(editor);
    // 绑定粘贴上传事件
    bindPaste(editor);

    bindSelect();
}

function bindSelect() {
    const bar = document.querySelector(".upload-bar");
    const input_file = document.querySelector("#ediotr-file-input");

    bar.addEventListener("click", (e) => {
        console.log(e)
        input_file.click();
    });

    input_file.addEventListener("change", (e) => {
        const files = e.target.files;
        for (let i = 0; i < files.length; i++) {
            uploadFile(files[i]);
        }

    })
}
// 点击预览
document.querySelector('.editor .preview').addEventListener('click', function (e) {

    // 点击预览就高亮显示代码
    if (hljs) {
        hljs.highlightAll();
    }

    editor.classList.remove('write');
    editor.classList.add('preview');
});

// 点击编辑
document.querySelector('.editor .write').addEventListener('click', function (e) {
    console.log("click preview")
    editor.classList.remove('preview');
    editor.classList.add('write');
    // insertText("test");
});

// 插入数据到编辑器中
function insertText(data) {
    let editor = document.querySelector(".editor .editor-input");
    let start = editor.selectionStart;
    let end = editor.selectionEnd;

    let value = editor.value;
    editor.value = value.slice(0, start) + data + value.slice(end);

    // 触发输入事件，调用转html代码
    var evt = new InputEvent('input', {
        inputType: 'insertText',
        data: value,
        dataTransfer: null,
        isComposing: false
    });
    editor.dispatchEvent(evt);
}


/**
* 拖拽上传
*/
function bindDrag(ele) {
    var obj = ele;
    obj.addEventListener("dragenter", handler, false);
    obj.addEventListener("dragover", handler, false);
    obj.addEventListener("drop", upload, false);
    function upload(e) {
        var e = e || window.event;
        handler(e);
        var files = e.dataTransfer.files;

        for (var i = 0, il = files.length; i < il; i++) {
            //如果是文件夹，就不处理
            if (files[i].size == 0) continue;
            uploadFile(files[i]);
            console.log(files[i]);
        }
    }

    function handler(e) {
        var e = e || window.event;
        e.preventDefault ? e.preventDefault() : e.returnValue = false;
        e.stopPropagation ? e.stopPropagation() : e.cancelBubble = true;
    }
}

var uploadStatus = false;
function bindPaste(ele) {
    //编辑器绑定粘贴事件，实现粘贴上传图片
    ele.addEventListener("paste", function (e) {
        var cbd = e.clipboardData;
        var ua = window.navigator.userAgent;

        // 如果是 Safari 直接 return
        if (!(e.clipboardData && e.clipboardData.items)) {
            return;
        }

        // Mac平台下Chrome49版本以下 复制Finder中的文件的Bug Hack掉
        if (cbd.items && cbd.items.length === 2 && cbd.items[0].kind === "string" && cbd.items[1].kind === "file" &&
            cbd.types && cbd.types.length === 2 && cbd.types[0] === "text/plain" && cbd.types[1] === "Files" &&
            ua.match(/Macintosh/i) && Number(ua.match(/Chrome\/(\d{2})/i)[1]) < 49) {
            return;
        }

        //如果在上传图片，不处理。
        if (uploadStatus) {
            return;
        }

        for (var i = 0; i < cbd.items.length; i++) {
            var item = cbd.items[i];
            if (item.kind == "file") {
                var blob = item.getAsFile();
                if (blob.size === 0) {
                    return;
                }
                // blob 就是从剪切板获得的文件 可以进行上传或其他操作
                uploadFile(blob);
            }
        }
    }, false);
}


/**
 * ajax方式上传图片
 * @param file 文件的内容
 */
function uploadFile(file) {

    let param = new FormData(); // 创建form对象
    param.append("FILE_NAME", file); // 通过append向form对象添加数据

    // param.append("chunk", "别的数据"); // 添加form表单中其他数据
    console.log(param.get("file")); // FormData私有类对象，访问不到，可以通过get判断值是否传进去

    let config = {
        headers: { "Content-Type": "multipart/form-data" }
    };

    axios.post(file_upload_url, param, config).then(res => {
        if (res.status != 200) {
            console.error("文件上传出错：" + res.status);
            return;
        }

        const data = res.data;

        // 处理上传错误
        if (data.code != 0) {
            alert(data.errMsg);
            return;
        }

        const file = data.src;


        if (data.isImage) {

            insertText("\r\n![" + data.alt + "](" + file_base_url + "/" + file + ")\r\n");
        } else {
            insertText("\r\n[" + data.alt + "](" + file_base_url + "/" + file + ")\r\n");
        }

    });
}
