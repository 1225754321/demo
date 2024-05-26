function base64ToFile(base64, fileName) {
    let arr = base64.split(",");
    let mime = arr[0].match(/:(.\*?);/)[1];
    let bstr = atob(arr[1]);
    let n = bstr.length;
    let u8arr = new Uint8Array(n);

    while (n--) {
        u8arr[n] = bstr.charCodeAt(n);
    }
    return new File([u8arr], fileName, { type: mime });
}

function fileToBase64(file) {
    return new Promise((resolve, reject) => {
        // 创建一个新的 FileReader 对象
        const reader = new FileReader();
        // 读取 File 对象
        reader.readAsDataURL(file);
        // 加载完成后
        reader.onload = function () {
            // 将读取的数据转换为 base64 编码的字符串
            const base64String = reader.result.split(",")[1];
            // 解析为 Promise 对象,并返回 base64 编码的字符串
            resolve(base64String);
        };

        // 加载失败时
        reader.onerror = function () {
            reject(new Error("Failed to load file"));
        };
    });
}

// ajax 上传单个文件对象
// url 为访问路径
// files 为上传文件数组
// success 为上传成功后执行方法
// fail 为上传文件失败执行的方法
function uploadFile(url, file, success, fail) {
    var form = new FormData();
    form.append("file", file);
    $.ajax({
        url: url,        //后台url
        data: form,
        cache: false,
        async: true,
        type: "POST",
        dataType: 'json',              //数据返回类型,可以是xml、json等
        processData: false,
        contentType: false,
        success: function (data) {      //成功,回调传来的success方法
            console.log(data);
        },
        error: function (er) {          //失败,回调函数
            console.log(er);
        }
    });
}

function urlParamToJson(url) {
    if (!url) {
        return {};
    }
    let json = {};
    url.substring(url.indexOf('?') + 1)
        .trim()
        .split('&')
        .forEach(item => json[item.split('=')[0]] = item.split('=')[1]);
    return json;
}