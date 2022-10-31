pub mod worker;

use std::collections::HashMap;

use napi::JsFunction;
use quick_js::{Context, JsValue};

#[napi]
pub fn register_quick_js_handler(function: JsFunction) {
    let function_string = function.coerce_to_string().unwrap().into_utf8().unwrap();
    let function_string = function_string.as_str().unwrap();
    
    let context = Context::new().unwrap();
    
    let global = make_js_global_req_obj();
    context.set_global("request", global).unwrap();

    let to_eval = format!("const recv = {}", function_string);

    context.eval(&to_eval).unwrap();

    let res = context.eval_as::<String>("recv(request)");
    println!("Result is {:?}", res);

}

fn make_js_global_req_obj() -> JsValue {
    // Build a basic request object with a url and method
    
    let mut req_obj = HashMap::<String, JsValue>::new();

    req_obj.insert("url".to_string(), JsValue::from("http://localhost:3000"));
    req_obj.insert("method".to_string(), JsValue::from("GET"));

    JsValue::from(req_obj)
}