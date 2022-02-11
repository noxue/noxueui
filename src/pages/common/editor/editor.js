
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
    insertText("test");
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
