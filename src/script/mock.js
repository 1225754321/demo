// tauri调度工具
if (window.__TAURI__) {
    var { invoke } = window.__TAURI__.tauri;
} else {
    var invoke = function test_invoke(name, data) {
        console.log("Test.invoke => ", name, data);
        return new Promise((resolve, reject) => {
            let m = mockData(name);
            if (m) {
                reject(m)
            }
            if (name.indexOf("err") != -1) {
                reject({
                    status: -1,
                    msg: "Err"
                });
                return;
            }
            if (name.indexOf("file") != -1) {
                sleep(1000)
            }
            return resolve({
                status: 0,
                msg: "OK"
            });
        });
    }
}

function runToData(method_name, data, options, handler) {
    console.log(method_name, data);
    invoke(method_name, { "req": data }).then(r => {
        console.log("tauri_util.invoke.then => ", r);
        handler.resolve({
            config: options,
            status: 200,
            // headers: { 'content-type': 'text/text' },
            response: {
                status: 0,
                msg: "ok",
                data: r
            }
        });
    }).catch(e => {
        console.log("tauri_util.invoke.err => ", e);
        handler.resolve({
            config: options,
            status: 200,
            // headers: { 'content-type': 'text/text' },
            response: {
                status: -1,
                msg: e,
            }
        });
    });
}

//调用mock方法模拟数据
ah.proxy({
    //请求发起前进入
    onRequest: (options, handler) => {
        console.log(options);
        let url = new URL("http://test.com" + options.url)
        let method_name = (options.method + url.pathname).replaceAll("/", "_").toLowerCase();
        let params = urlParamToJson(url.search);
        let bodys = {};
        try {
            bodys = JSON.parse(options.body);
            runToData(method_name, { params: params, bodys: bodys }, options, handler);
        } catch (e) {
            console.log(e.message);
            if (options.body instanceof FormData) {
                for (let file of options.body.values()) {
                    console.log(file);
                    let name = file.path;
                    fileToBase64(file).then(r => {
                        bodys[name] = r;
                        runToData(method_name, { params: params, bodys: bodys }, options, handler);
                    });
                }
            }
        }

    },
    //请求发生错误时进入，比如超时；注意，不包括http状态码错误，如404仍然会认为请求成功
    onError: (err, handler) => {
        console.log("err => ", err)
        handler.next(err)
    },
    //请求成功后进入
    onResponse: (response, handler) => {
        console.log("response => ", response)
        handler.next(response)
    }
});
