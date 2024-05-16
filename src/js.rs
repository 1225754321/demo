use boa_engine::{Context, Source};
use serde_json::Value;

pub fn run_js(source: &str) -> Value {
    let mut context: Context = Context::default();
    let result = context.eval(Source::from_bytes(source)).unwrap();
    result.to_json(&mut context).unwrap()
}
