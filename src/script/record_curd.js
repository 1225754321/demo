var record_curd = {
    "type": "page",
    "title": "",
    "body": [
        {
            "type": "crud",
            "syncLocation": false,
            "api": {
                "method": "post",
                "url": "/records",
                "requestAdaptor": (r, c) => {
                    console.log(c);
                    ct = undefined
                    if (c.create_time != undefined) {
                        des = c.create_time.split(",")
                        ct = {
                            start_time: new Date(des[0] * 1000),
                            end_time: new Date(des[1] * 1000)
                        }
                    }
                    ut = undefined
                    if (c.update_time != undefined) {
                        des = c.create_time.split(",")
                        ut = {
                            start_time: new Date(des[0] * 1000),
                            end_time: new Date(des[1] * 1000)
                        }
                    }
                    return {
                        ...r,
                        data: {
                            ...r.data,
                            "per_page": c.perPage,
                            "order_by": c.orderBy,
                            "order_dir": c.orderDir,
                            "data": {
                                "id": c.id,
                                "content": c.content,
                                "quotes": c.quotes ? c.quotes.split(",") : [],
                                "referenceds": c.referenceds ? c.referenceds.split(",") : [],
                                "labels": c.labels ? c.labels.split(",") : [],
                            },
                            "create_time": ct,
                            "update_time": ut,
                        }
                    };
                }
            },
            // "autoGenerateFilter": {
            //     "columnsNum": 2,
            //     "showBtnToolbar": false
            // },
            "headerToolbar": ["bulkActions", "pagination"],
            "columns": [
                {
                    "name": "id",
                    "label": "ID",
                    "type": "text",
                    "sortable": true,
                    "searchable": true,
                    "id": "u:292727c6920a"
                },
                {
                    "name": "content",
                    "label": "内容",
                    "type": "markdown",
                    "searchable": true,
                    "id": "u:cef0b36e020b"
                },
                {
                    "type": "text",
                    "name": "labels",
                    "label": "标签组",
                    "searchable": {
                        "type": "input-tag",
                        "label": "标签组",
                        "placeholder": "请选择标签",
                        "options": [
                            "Aaron Rodgers",
                            "Tom Brady",
                            "Charlse Woodson",
                            "Aaron Jones"
                        ]
                    },
                    "id": "u:cf20988feb18"
                },
                {
                    "type": "text",
                    "name": "quotes",
                    "label": "引用情况",
                    "toggled": false,
                    "searchable": {
                        "type": "input-tag",
                        "label": "引用",
                        "placeholder": "请选择引用",
                        "options": [
                            "Aaron Rodgers",
                            "Tom Brady",
                            "Charlse Woodson",
                            "Aaron Jones"
                        ]
                    },
                    "id": "u:18d01b7aee38"
                },
                {
                    "type": "text",
                    "label": "被引用情况",
                    "name": "referenceds",
                    "toggled": false,
                    "searchable": {
                        "type": "input-tag",
                        "label": "引用",
                        "placeholder": "请选择引用",
                        "options": [
                            "Aaron Rodgers",
                            "Tom Brady",
                            "Charlse Woodson",
                            "Aaron Jones"
                        ]
                    },
                    "id": "u:d61bb65cff32"
                },
                {
                    "type": "date",
                    "label": "创建时间",
                    "toggled": false,
                    "sortable": true,
                    "name": "create_time",
                    "searchable": {
                        "type": "input-date-range",
                        "label": "创建时间范围"
                    },
                    "id": "u:1dce08bd6b78"
                },
                {
                    "type": "date",
                    "label": "修改时间",
                    "toggled": false,
                    "sortable": true,
                    "name": "update_time",
                    "searchable": true,
                    "id": "u:ae3325cdede3"
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
                                "body": {
                                    "type": "form",
                                    "api": "xxx/update",
                                    "body": [
                                        {
                                            "name": "id",
                                            "label": "ID",
                                            "type": "input-text"
                                        },
                                        {
                                            "name": "content",
                                            "label": "内容",
                                            "type": "input-text"
                                        },
                                        {
                                            "name": "labels",
                                            "label": "标签组",
                                            "type": "input-text"
                                        },
                                        {
                                            "name": "quotes",
                                            "label": "引用情况",
                                            "type": "input-text"
                                        },
                                        {
                                            "label": "被引用情况",
                                            "name": "referenceds",
                                            "type": "input-text"
                                        },
                                        {
                                            "label": "创建时间",
                                            "name": "create_time",
                                            "type": "input-date"
                                        },
                                        {
                                            "label": "修改时间",
                                            "name": "update_time",
                                            "type": "input-date"
                                        }
                                    ]
                                }
                            },
                            "id": "u:416549b859dc"
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
                                "body": {
                                    "type": "form",
                                    "api": "xxx/update",
                                    "body": [
                                        {
                                            "name": "id",
                                            "label": "ID",
                                            "type": "static"
                                        },
                                        {
                                            "name": "content",
                                            "label": "内容",
                                            "type": "static"
                                        },
                                        {
                                            "name": "labels",
                                            "label": "标签组",
                                            "type": "static"
                                        },
                                        {
                                            "name": "quotes",
                                            "label": "引用情况",
                                            "type": "static"
                                        },
                                        {
                                            "label": "被引用情况",
                                            "name": "referenceds",
                                            "type": "static"
                                        },
                                        {
                                            "label": "创建时间",
                                            "name": "create_time",
                                            "type": "static"
                                        },
                                        {
                                            "label": "修改时间",
                                            "name": "update_time",
                                            "type": "static"
                                        }
                                    ]
                                }
                            },
                            "id": "u:f4d99186dfa1"
                        },
                        {
                            "type": "button",
                            "label": "删除",
                            "actionType": "ajax",
                            "level": "link",
                            "className": "text-danger",
                            "confirmText": "确定要删除？",
                            "api": {
                                "method": "post",
                                "url": "/record"
                            },
                            "editorSetting": {
                                "behavior": "delete"
                            },
                            "id": "u:af994b320404"
                        }
                    ],
                    "id": "u:7ae5ec1ec8ff"
                }
            ],
            "bulkActions": [
                {
                    "type": "button",
                    "level": "danger",
                    "label": "批量删除",
                    "actionType": "ajax",
                    "confirmText": "确定要删除？",
                    "api": "/xxx/batch-delete",
                    "editorSetting": {
                        "behavior": "bulkDelete"
                    },
                    "id": "u:85a62cf0ced7"
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
                            "id": "uuid_add",
                            "api": {
                                "method": "post",
                                "url": "/record"
                            },
                            data: {
                                id: "",
                                content: "",
                                labels: "",
                            },
                            "body": [
                                {
                                    "type": "static-markdown",
                                    "name": "content",
                                },
                                {
                                    "type": "divider"
                                },
                                {
                                    "label": "内容",
                                    "type": "editor",
                                    "name": "content",
                                    "required": true,
                                    "language": "markdown"
                                },
                                {
                                    "name": "id",
                                    "label": "ID",
                                    "required": true,
                                    "clearable": true,
                                    "addOn": {
                                        "type": "button",
                                        "label": "随机生成",
                                        "actionType": "custom",
                                        "onClick": (event, props) => {
                                            console.log(props);
                                            console.log(event);
                                            console.log(props.onAction(event, {
                                                "actionType": "setValue",
                                                "componentId": "uuid_add",
                                                "args": {
                                                    "value": {
                                                        id: "dsadsadasdsad",
                                                        content: "${content}",
                                                        labels: "${labels}",
                                                    }
                                                }
                                            }));
                                        }
                                    },
                                    "type": "input-text"
                                },
                                {
                                    "type": "select",
                                    "name": "labels",
                                    "required": true,
                                    "multiple": true,
                                    "clearable": true,
                                    "autoComplete": {
                                        "method": "post",
                                        "url": "/labels",
                                        "requestAdaptor": (r, c) => {
                                            console.log(r);
                                            console.log(c);
                                            return {
                                                ...r,
                                                data: "\"" + c.term + "\""
                                            };
                                        }
                                    },
                                    "label": "标签组"
                                },
                            ]
                        }
                    },
                    "id": "u:54f7b9bb5dff"
                },
                "bulkActions"
            ],
            "id": "u:409cc08662cd"
        }
    ],
    "id": "u:63683c3d3018",
    "asideResizor": false,
    "pullRefresh": {
        "disabled": true
    },
    "aside": [],
    "toolbar": [],
    "regions": [
        "body"
    ]
};