<?php
header("Access-Control-Allow-Origin: *"); // 允许任意域名发起的跨域请求
header('Content-Type: application/json');

$name = "file";

if ($_FILES[$name]["error"] > 0) {
    echo json_encode([
        "code" => -1,
        "errMsg" => "Return Code: " . $_FILES["file"]["error"],
    ]);
} else {
    $ext = substr($_FILES[$name]["name"], strrpos($_FILES[$name]["name"], '.') + 1);

    if (!in_array($ext, ['jpg', 'png', 'gif', 'bmp', 'zip', 'rar'])) {
        echo json_encode([
            "code" => -1,
            "errMsg" => "只支持上传jpg,gif,bmp,png,zip,rar文件，不支持：" . $ext,
        ]);
        exit;
    }

    $fileName = "target/" . rand(1000, 1000000) . "." . $ext;
    if (file_exists($fileName)) {
        echo $_FILES[$name]["name"] . " already exists. ";
    } else {
        move_uploaded_file($_FILES[$name]["tmp_name"],
            $fileName);

        echo json_encode([
            "code" => 0,
            "src" => $fileName,
            "alt" => $_FILES[$name]["name"],
            "isImage" => in_array(strtolower($ext), ["jpg", "gif", "jpeg", "bmp", "png"]),
            "errMsg" => "",
        ]);
    }
}
