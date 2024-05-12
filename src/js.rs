use deno_core::v8;
use deno_core::{JsRuntime, RuntimeOptions};
use serde_json::Value;
use serde_v8;

pub fn run_js(source: String) -> Value {
    // 创建JavaScript运行时
    let mut runtime = JsRuntime::new(RuntimeOptions {
        ..Default::default()
    });
    // 执行JavaScript代码
    if let Ok(result) = runtime.execute_script("run", source) {
        let mut scope: v8::HandleScope<v8::Context> = runtime.handle_scope();
        let scope: &mut v8::HandleScope<v8::Context> = scope.as_mut();
        let local = v8::Local::new(scope, &result);
        if let Ok(json) = convert_to_json(scope, local) {
            return json;
        }
    }
    Value::default()
}

fn convert_to_json(
    scope: &mut v8::HandleScope<v8::Context>,
    value: v8::Local<v8::Value>,
) -> Result<serde_json::Value, serde_v8::Error> {
    let serde_value = serde_v8::from_v8(scope, value)?;
    Ok(serde_value)
}
