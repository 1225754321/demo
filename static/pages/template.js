var placeholder = `<!DOCTYPE html>
<html>

<head>
    <title>Demo</title>
    <!-- 这里提供amis脚本 -->
    <link rel="stylesheet" href="/static/amis/sdk.css" />
    <link rel="stylesheet" href="/static/amis/helper.css" />
    <link rel="stylesheet" href="/static/amis/iconfont.css" />
    <!-- 这是默认主题所需的，如果是其他主题则不需要 -->
    <!-- 从 1.1.0 开始 sdk.css 将不支持 IE 11，如果要支持 IE11 请引用这个 css，并把前面那个删了 -->
    <!-- <link rel="stylesheet" href="sdk-ie11.css" /> -->
    <!-- 不过 amis 开发团队几乎没测试过 IE 11 下的效果，所以可能有细节功能用不了，如果发现请报 issue -->
    <style>
        html,
        body,
        .app-wrapper {
            position: relative;
            width: 100%;
            height: 100%;
            margin: 0;
            padding: 0;
        }
    </style>
    <!-- 这里提供jq脚本 -->
    <script src="/static/js/jquery-3.7.1.min.js"></script>
</head>

<body>
    <h1 id="a1">Demo</h1>
    <div id="root" class="app-wrapper"></div>
    <a id="a1">点击这里</a>
    <!-- 构建template必须要加上下面这一行
        这一行会替换为 <script>var req={};script;/script>
        其中req是请求参数, 可以在后续调用中使用,
        ResetScript是用户自定义的js脚本, 用于为temp提供动态数据
    -->
    <script>{ { DATA } };</script>
    <script src="/static/amis/sdk.js"></script>
    <script>
    (function () {
        $("#a1").text("Hello world Demo!");
        let amis = amisRequire('amis/embed');
        // 通过替换下面这个配置来生成不同页面
        let amisJSON = {
            "type": "page",
            "body": {
                "type": "json",
                "levelExpand": 3,
                "value": req,
            }
        };
        let amisScoped = amis.embed('#root', amisJSON);
    })();
    </script>
</body>

</html>`

var templateJSON = {
    "type": "crud",
    "syncLocation": false,
    "api": {
        "method": "get",
        "url": "/control/template"
    },
    "columns": [
        {
            "name": "name",
            "label": "name",
            "type": "text",
            "searchable": true,
            "id": "u:a6c20dfcdbfb"
        },
        {
            "name": "templage_html",
            "label": "模板内容",
            "type": "editor",
            "language": "html",
            "disabled": true,
            "id": "u:3b7068dfb9ce"
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
                                "url": "/control/template",
                                "method": "put"
                            },
                            "body": [
                                {
                                    "name": "name",
                                    "label": "name",
                                    "type": "static"
                                },
                                {
                                    "name": "templage_html",
                                    "label": "模板内容",
                                    "type": "editor",
                                    "language": "html",
                                }
                            ]
                        }
                    },
                    "id": "u:4002f3d52051"
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
                                    "name": "templage_html",
                                    "label": "模板内容",
                                    "type": "editor",
                                    "language": "html",
                                    "disabled": true,
                                }
                            ]
                        }
                    },
                    "id": "u:8ef7b4af7281"
                },
                {
                    "type": "button",
                    "label": "删除",
                    "actionType": "ajax",
                    "level": "link",
                    "className": "text-danger",
                    "confirmText": "确定要删除？",
                    "api": {
                        "url": "/control/template",
                        "method": "delete",
                        "data": {
                            "name": "${name}",
                            "templage_html": "${templage_html}"
                        }
                    },
                    "editorSetting": {
                        "behavior": "delete"
                    },
                    "id": "u:53625cd9ec5e"
                }
            ],
            "id": "u:bbcf08819adb"
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
                        "url": "/control/template"
                    },
                    "body": [
                        {
                            "type": "input-text",
                            "name": "name",
                            "label": "name"
                        },
                        {
                            "type": "editor",
                            "language": "html",
                            "name": "templage_html",
                            "value": placeholder,
                            "label": "模板内容",
                        }
                    ]
                }
            },
            "id": "u:dc19a565fa1c"
        },
        "bulkActions"
    ],
    "id": "u:8968d6f6e859",
};