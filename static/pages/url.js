var urlJSON = {
    "type": "service",
    "id": "apiJSONData",
    "data": {
        "switch": false,
    },
    "body": {
        "type": "container",
        "body": [
            {
                "name": "switch",
                "type": "switch",
                "option": "url配置"
            },
            {
                "type": "crud",
                "visibleOn": "data.switch",
                "syncLocation": false,
                "api": {
                    "method": "get",
                    "url": "/control/html"
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
                        "label": "template",
                        "name": "template",
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
                                            "url": "/control/html?key=${key}",
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
                                                "type": "static"
                                            },
                                            {
                                                "type": "input-text",
                                                "name": "template",
                                                "value": "",
                                                "source": {
                                                    "method": "get",
                                                    "url": "/control/template",
                                                    "responseData": {
                                                        "options": "${items|pick:label~name,value~name}"
                                                    }
                                                },
                                                "label": "template"
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
                                                "actionType": "url",
                                                "args": {
                                                    "url": "${path}",
                                                    "blank": true,
                                                }
                                            }
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
                                    "url": "/control/html",
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
                                    "url": "/control/html"
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
                                        "value": "GET",
                                        "type": "static"
                                    }, {
                                        "type": "input-text",
                                        "name": "template",
                                        "value": "",
                                        "source": {
                                            "method": "get",
                                            "url": "/control/template",
                                            "responseData": {
                                                "options": "${items|pick:label~name,value~name}"
                                            }
                                        },
                                        "label": "template"
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
                                        "value": true,
                                        "hidden": true,
                                        "label": "is_html"
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
                "title": "url接口列表"
            },
        ],
    }
};