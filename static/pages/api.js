var apiJSON = {
    "type": "crud",
    "syncLocation": false,
    "api": {
        "method": "get",
        "url": "/control/api"
    },
    "columns": [
        {
            "name": "key",
            "type": "text",
            "id": "u:17e41f6e80e4",
            "hidden": true,
            "placeholder": "-"
        }, {
            "name": "name",
            "label": "name",
            "type": "text",
            "id": "u:17e41f6e80e4",
            "searchable": true,
            "placeholder": "-"
        },
        {
            "name": "path",
            "label": "path",
            "type": "text",
            "searchable": true,
            "id": "u:7e6cc481606d"
        },
        {
            "type": "text",
            "label": "code",
            "name": "code",
            "id": "u:9e07335979d1",
            "placeholder": "-",
            "quickEdit": false
        },
        {
            "type": "text",
            "label": "method",
            "name": "method",
            "id": "u:30c00958590a",
            "placeholder": "-"
        },
        {
            "type": "editor",
            "language": "javascript",
            "label": "script",
            "name": "script",
            "disabled": true,
            "id": "u:c6625dca954e",
            "placeholder": "-"
        },
        {
            "type": "editor",
            "language": "javascript",
            "label": "header_script",
            "name": "header_script",
            "hidden": true,
            "id": "u:c6625dca954e",
            "placeholder": "-"
        }, {
            "type": "editor",
            "language": "javascript",
            "label": "code_script",
            "name": "code_script",
            "hidden": true,
            "id": "u:c6625dca954e",
            "placeholder": "-"
        },
        {
            "type": "operation",
            "label": "操作",
            "buttons": [
                {
                    "label": "编辑",
                    "type": "button",
                    "actionType": "dialog",
                    "level": "link",
                    "editorSetting": {
                        "behavior": "update"
                    },
                    "dialog": {
                        "title": "编辑",
                        "size": "full",
                        "body": {
                            "type": "form",
                            "api": {
                                "url": "/control/api?key=${key}",
                                "method": "put"
                            },
                            "body": [
                                {
                                    "name": "key",
                                    "label": "key",
                                    "hidden": true,
                                    "type": "static"
                                },
                                {
                                    "name": "name",
                                    "label": "name",
                                    "type": "input-text"
                                },
                                {
                                    "name": "path",
                                    "label": "path",
                                    "type": "input-text"
                                },
                                {
                                    "label": "code",
                                    "name": "code",
                                    "type": "input-number"
                                },
                                {
                                    "label": "method",
                                    "name": "method",
                                    "type": "select",
                                    "options": [
                                        {
                                            "label": "GET",
                                            "value": "GET"
                                        },
                                        {
                                            "label": "HEAD",
                                            "value": "HEAD"
                                        },
                                        {
                                            "label": "POST",
                                            "value": "POST"
                                        },
                                        {
                                            "label": "PUT",
                                            "value": "PUT"
                                        },
                                        {
                                            "label": "DELETE",
                                            "value": "DELETE"
                                        },
                                        {
                                            "label": "CONNECT",
                                            "value": "CONNECT"
                                        },
                                        {
                                            "label": "OPTIONS",
                                            "value": "OPTIONS"
                                        },
                                        {
                                            "label": "TRACE",
                                            "value": "TRACE"
                                        },
                                        {
                                            "label": "PATCH",
                                            "value": "PATCH"
                                        }
                                    ]
                                },
                                {
                                    "label": "script",
                                    "name": "script",
                                    "type": "editor",
                                    "language": "javascript"
                                },
                                {
                                    "label": "header_script",
                                    "name": "header_script",
                                    "type": "editor",
                                    "language": "javascript"
                                },
                                {
                                    "label": "code_script",
                                    "name": "code_script",
                                    "type": "editor",
                                    "language": "javascript"
                                },
                                {
                                    "type": "input-text",
                                    "name": "is_html",
                                    "value": false,
                                    "hidden": true,
                                    "label": "is_html"
                                },
                                {
                                    "type": "input-text",
                                    "name": "template",
                                    "value": "",
                                    "hidden": true,
                                    "label": "template"
                                },
                            ]
                        }
                    },
                    "id": "u:6c2c4343ef40"
                },
                {
                    "label": "查看",
                    "type": "button",
                    "actionType": "dialog",
                    "level": "link",
                    "editorSetting": {
                        "behavior": "view"
                    },
                    "dialog": {
                        "title": "查看详情",
                        "size": "full",
                        "body": {
                            "type": "form",
                            "body": [
                                {
                                    "name": "name",
                                    "label": "name",
                                    "type": "static"
                                },
                                {
                                    "name": "path",
                                    "label": "path",
                                    "type": "static"
                                },
                                {
                                    "label": "code",
                                    "name": "code",
                                    "type": "static"
                                },
                                {
                                    "label": "method",
                                    "name": "method",
                                    "type": "static"
                                },
                                {
                                    "label": "script",
                                    "name": "script",
                                    "disabled": true,
                                    "type": "editor",
                                    "language": "javascript"
                                },
                                {
                                    "label": "header_script",
                                    "name": "header_script",
                                    "disabled": true,
                                    "type": "editor",
                                    "language": "javascript"
                                },
                                {
                                    "label": "code_script",
                                    "name": "code_script",
                                    "disabled": true,
                                    "type": "editor",
                                    "language": "javascript"
                                }
                            ]
                        }
                    },
                    "id": "u:39dc62180f5f"
                },
                {
                    "type": "button",
                    "label": "测试",
                    "level": "link",
                    "onEvent": {
                        "click": {
                            "actions": [
                                {
                                    "actionType": "dialog",
                                    "dialog": {
                                        "type": "dialog",
                                        "title": "模态弹窗",
                                        "id": "dialog_001",
                                        "data": {
                                            "ApiUrl": "${path}",
                                            "Method": "${method}",
                                        },
                                        "body": [
                                            {
                                                "type": "tpl",
                                                "tpl": "<p>测试请求配置</p>",
                                                "inline": false
                                            },
                                            {
                                                "type": "input-text",
                                                "name": "query",
                                                "label": "query段",
                                                "value": "?q=sdf&c=123",
                                            },
                                            {
                                                "label": "请求数据body配置,最终变量就是请求数据,使用eval()",
                                                "visibleOn": "data.Method.toUpperCase() != 'GET'",
                                                "name": "script",
                                                "type": "editor",
                                                "value": "var data =\"{}\";data",
                                                "language": "javascript"
                                            }
                                        ],
                                        "onEvent": {
                                            "confirm": {
                                                "actions": [
                                                    {
                                                        "actionType": "custom",
                                                        "script": function (context, doAction, event) {
                                                            console.log("event =>", event);
                                                            doAction({
                                                                "actionType": "ajax",
                                                                "api": {
                                                                    "url": event.data.ApiUrl + event.data.query,
                                                                    "method": event.data.Method,
                                                                    "data": eval(event.data.script),
                                                                    "messages": {
                                                                    },
                                                                    "adaptor": function (payload, response, api, context) {
                                                                        doAction({
                                                                            "actionType": "toast",
                                                                            "args": {
                                                                                "msg": "成功了！欧耶"
                                                                            }
                                                                        });
                                                                        console.log("payload => ", payload)
                                                                        doAction({
                                                                            "actionType": "toast",
                                                                            "args": {
                                                                                "msg": JSON.stringify(payload)
                                                                            }
                                                                        });
                                                                        event.stopPropagation();
                                                                        return { ...payload, status: 0 };
                                                                    }
                                                                }
                                                            });
                                                            event.stopPropagation();
                                                        }
                                                    },
                                                ]
                                            },
                                            "cancel": {
                                                "actions": [
                                                ]
                                            }
                                        }
                                    }
                                },

                            ]
                        }
                    }
                },
                {
                    "type": "button",
                    "label": "删除",
                    "actionType": "ajax",
                    "level": "link",
                    "className": "text-danger",
                    "confirmText": "确定要删除？",
                    "api": {
                        "method": "delete",
                        "url": "/control/api",
                        "data": {
                            "key": "",
                            "name": "",
                            "method": "${method}",
                            "path": "${path}",
                            "code": 1,
                            "script": "",
                            "header_script": "",
                            "code_script": "",
                            "is_html": true,
                            "template": ""
                        }
                    },
                    "editorSetting": {
                        "behavior": "delete"
                    },
                    "id": "u:1dcd89bb757d"
                }
            ],
            "id": "u:05f3cd317ba1"
        }
    ],
    "itemActions": [],
    "headerToolbar": [
        {
            "label": "新增",
            "type": "button",
            "actionType": "dialog",
            "level": "primary",
            "editorSetting": {
                "behavior": "create"
            },
            "dialog": {
                "title": "新增",
                "size": "full",
                "body": {
                    "type": "form",
                    "api": {
                        "method": "post",
                        "url": "/control/api"
                    },
                    "body": [
                        {
                            "type": "input-text",
                            "name": "key",
                            "value": "",
                            "hidden": true,
                            "label": "key"
                        },
                        {
                            "type": "input-text",
                            "name": "name",
                            "value": "",
                            "label": "name"
                        },
                        {
                            "type": "input-text",
                            "name": "path",
                            "value": "/",
                            "label": "path"
                        },
                        {
                            "type": "input-number",
                            "name": "code",
                            "value": 200,
                            "label": "code"
                        },
                        {
                            "name": "method",
                            "label": "method",
                            "value": "POST",
                            "type": "select",
                            "options": [
                                {
                                    "label": "GET",
                                    "value": "GET"
                                },
                                {
                                    "label": "HEAD",
                                    "value": "HEAD"
                                },
                                {
                                    "label": "POST",
                                    "value": "POST"
                                },
                                {
                                    "label": "PUT",
                                    "value": "PUT"
                                },
                                {
                                    "label": "DELETE",
                                    "value": "DELETE"
                                },
                                {
                                    "label": "CONNECT",
                                    "value": "CONNECT"
                                },
                                {
                                    "label": "OPTIONS",
                                    "value": "OPTIONS"
                                },
                                {
                                    "label": "TRACE",
                                    "value": "TRACE"
                                },
                                {
                                    "label": "PATCH",
                                    "value": "PATCH"
                                }
                            ]
                        },
                        {
                            "type": "editor",
                            "language": "javascript",
                            "name": "script",
                            "value": "",
                            "label": "script"
                        },
                        {
                            "type": "editor",
                            "language": "javascript",
                            "name": "header_script",
                            "value": "",
                            "label": "header_script"
                        },
                        {
                            "type": "editor",
                            "language": "javascript",
                            "name": "code_script",
                            "value": "",
                            "label": "code_script"
                        },
                        {
                            "type": "input-text",
                            "name": "is_html",
                            "value": false,
                            "hidden": true,
                            "label": "is_html"
                        },
                        {
                            "type": "input-text",
                            "name": "template",
                            "value": "",
                            "hidden": true,
                            "label": "template"
                        },
                    ]
                }
            },
            "id": "u:69f8425901e7"
        },
        "bulkActions"
    ],
    "id": "u:e6972ed72056",
    "perPageAvailable": [
        10
    ],
    "messages": {},
    "primaryField": "name",
};