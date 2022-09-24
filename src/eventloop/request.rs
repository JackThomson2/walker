use std::ptr;

use napi::{sys::napi_ref, Env, Result, NapiRaw, JsObject};

use crate::request::RequestBlob;

pub struct RequestData {
    node_function: napi_ref,
    function_data: RequestBlob
}

impl RequestData {
    pub fn new(node_function: napi_ref, function_data: RequestBlob) -> Self {
        Self { node_function, function_data }
    }

    /// HUGELY UNSAFE!! This needs to be called ONLY
    /// from the node event loop, (hopefully the ENV restriction stops this happening)
    pub unsafe fn call_method(self, env: &Env) -> Result<()> {
        let mut raw_ref = ptr::null_mut();
        let _ = napi::sys::napi_get_reference_value(env.raw(), self.node_function, &mut raw_ref);

        let mut js_obj: JsObject = env.create_object()?;
        env.wrap(&mut js_obj, self.function_data)?;

        let undef = env.get_undefined()?.raw();
        let params = vec![(&js_obj).raw()];

        let mut resulting = ptr::null_mut();
        napi::sys::napi_call_function(env.raw(), undef, raw_ref, 1, params.as_ptr(), &mut resulting);

        Ok(())
    }
}