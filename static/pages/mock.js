var mockJSON = {
    "type": "service",
    "id": "mockJSONData",
    "data": {
        "js_editor": "var data = {code:123};data",
        "headers": [{ "key": "Content-Type", "value": "application/json; charset=utf-8" }],
        "req_editor": "{}",
        "querys": "q=123&h=test",
        "resp_editor": "",
        "switch": false,
    },
    "body": {
        "type": "flex",
        "className": "p-1",
        "items": [
            {
                "name": "switch",
                "type": "switch",
                "option": "mock测试"
            },
            {
                "type": "editor",
                "visibleOn": "data.switch",
                "label": "js脚本",
                "name": "js_editor",
                "id": "u:ca38fc30ca2a",
                "language": "javascript"
            },
            {
                "type": "input-table",
                "visibleOn": "data.switch",
                "name": "headers",
                "label": "请求头",
                "columns": [
                    {
                        "label": "请求头Key",
                        "name": "key",
                        "quickEdit": {
                            "type": "input-text",
                            "name": "key"
                        },
                        "type": "text",
                        "id": "u:633c4df9f129"
                    },
                    {
                        "label": "请求头Value",
                        "name": "value",
                        "quickEdit": {
                            "type": "input-text",
                            "name": "value"
                        },
                        "type": "text",
                        "id": "u:f3376f8f0d02"
                    }
                ],
                "addable": true,
                "footerAddBtn": {
                    "label": "新增",
                    "icon": "fa fa-plus",
                    "id": "u:16d89b7f80e8"
                },
                "strictMode": true,
                "id": "u:97204756993f",
                "minLength": 0,
                "copyable": false,
                "editable": true,
                "removable": true,
                "showIndex": false,
                "perPage": ""
            },
            {
                "type": "editor",
                "visibleOn": "data.switch",
                "label": "请求体",
                "name": "req_editor",
                "id": "u:5bebc3517b08",
                "language": "javascript"
            },
            {
                "type": "input-text",
                "visibleOn": "data.switch",
                "label": "query参数",
                "name": "querys",
                "id": "u:b46df2fb2905",
                "clearable": true,
                "showCounter": false,
                "static": false
            },
            {
                "type": "container",
                "visibleOn": "data.switch",
                "body": [
                    {
                        "type": "button-group",
                        "buttons": [
                            {
                                "type": "button",
                                "label": "测试",
                                "onEvent": {
                                    "click": {
                                        "actions": [
                                            {
                                                "actionType": "custom",
                                                "script": function (context, doAction, event) {
                                                    console.log("context => ", context);
                                                    console.log("doAction => ", doAction);
                                                    console.log("event => ", event);
                                                    console.log("event.setData => ", event.setData);
                                                    console.log("event.stopPropagation => ", event.stopPropagation);
                                                    var headers = {};
                                                    for (const key in event.data.headers) {
                                                        if (Object.hasOwnProperty.call(event.data.headers, key)) {
                                                            const element = event.data.headers[key];
                                                            headers[key] = element;
                                                        }
                                                    }
                                                    doAction({
                                                        "actionType": "ajax",
                                                        "api": {
                                                            "url": "/mockjs?" + event.data.querys,
                                                            "method": "post",
                                                            "headers": headers,
                                                            "data": {
                                                                "req": event.data.req_editor,
                                                                "data": event.data.js_editor
                                                            },
                                                            "messages": {
                                                                "success": "请求成功了！欧耶",
                                                                "failed": "失败了呢。。"
                                                            },
                                                            "adaptor": function (payload, response, api, context) {
                                                                console.log("payload => ", payload);
                                                                console.log("response => ", response);
                                                                console.log("api => ", api);
                                                                console.log("context => ", context);
                                                                event.setData({ ...event.data, "resp_editor": JSON.stringify(payload.data) });
                                                                doAction({
                                                                    "actionType": "setValue",
                                                                    "componentId": "mockJSONData",
                                                                    "args": {
                                                                        "value": event.data
                                                                    }
                                                                });
                                                                console.log("event.data => ", event.data);
                                                                event.stopPropagation();
                                                                return {
                                                                    ...payload
                                                                };
                                                            }
                                                        }
                                                    });
                                                    event.stopPropagation();
                                                }
                                            }
                                        ]
                                    }
                                },
                                "id": "u:104a929e7aad"
                            },
                            {
                                "type": "button",
                                "label": "重置",
                                "onEvent": {
                                    "click": {
                                        "actions": [
                                            {
                                                "actionType": "custom",
                                                "script": function (context, doAction, event) {
                                                    console.log("重置 event.data => ", event.data);
                                                    event.setData({
                                                        ...event.data, ...{
                                                            "js_editor": "var data = {code:123};data",
                                                            "headers": [{ "key": "Content-Type", "value": "application/json; charset=utf-8" }],
                                                            "req_editor": "{}",
                                                            "querys": "q=123&h=test",
                                                            "resp_editor": "",
                                                        }
                                                    });
                                                    console.log("重置2 event.data => ", event.data);
                                                    doAction({
                                                        "actionType": "setValue",
                                                        "componentId": "mockJSONData",
                                                        "args": {
                                                            "value": event.data
                                                        }
                                                    });
                                                    event.stopPropagation();
                                                }
                                            }
                                        ]
                                    }
                                },
                                "id": "u:7991aef6aceb"
                            }
                        ],
                        "id": "u:858fe19fc5c0",
                        "tiled": true,
                        "btnLevel": "primary",
                        "size": "md",
                        "btnClassName": "m-sm b-primary"
                    }
                ],
                "size": "xs",
                "style": {
                    "position": "static",
                    "display": "block",
                    "flex": "1 1 auto",
                    "flexGrow": 1
                },
                "wrapperBody": false,
                "isFixedHeight": false,
                "isFixedWidth": false,
                "id": "u:0f44777c69b8"
            },
            {
                "type": "container",
                "visibleOn": "data.switch",
                "body": [
                    {
                        "type": "editor",
                        "label": "测试结果",
                        "name": "resp_editor",
                        "id": "u:9449d7b8adaf",
                        "language": "javascript"
                    }
                ],
                "size": "xs",
                "style": {
                    "position": "static",
                    "display": "block",
                    "flex": "1 1 auto",
                    "flexGrow": 1
                },
                "wrapperBody": false,
                "isFixedHeight": false,
                "isFixedWidth": false,
                "id": "u:ee35c275788e"
            }
        ],
        "style": {
            "position": "relative",
            "inset": "auto",
            "flexWrap": "nowrap",
            "flexDirection": "column",
            "alignItems": "stretch"
        },
        "id": "u:d541fc363a2a",
        "isFixedHeight": false,
        "isFixedWidth": false,
        "themeCss": {
            "baseControlClassName": {
                "border:default": {
                    "top-border-style": "var(--borders-style-4)",
                    "left-border-style": "var(--borders-style-4)",
                    "right-border-style": "var(--borders-style-4)",
                    "bottom-border-style": "var(--borders-style-4)"
                },
                "background:default": "var(--colors-brand-10)"
            }
        }
    }
};