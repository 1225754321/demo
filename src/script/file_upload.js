var file_upload = {
    "type": "input-group",
    "body": [{
        "type": "input-file",
        "name": "files",
        "label": false,
        "useChunk": false,
        "mode": "horizontal",
        "accept": "*",
        "receiver": "/upload",
        "multiple": true,
        "downloadUrl": false,
        "joinValues": false,
        "id": "clear_text",
    }, {
        "type": "button",
        "label": "复制上传文件列表的markdown格式",
        "onEvent": {

            "click": {
                "actions": [
                    {
                        "actionType": "copy",
                        "args": {
                            "content": "${JOIN(ARRAYMAP(files, item => item.value), '\\n')}"
                        }
                    }
                ]
            }
        }
    }, {
        "type": "button",
        "label": "清空文件列表",
        "onEvent": {
            "click": {
                "actions": [
                    {
                        "actionType": "clear",
                        "componentId": "clear_text"
                    }
                ]
            }
        }
    }]
};