use napi::Result;
use serde_json::Value;

use crate::{
    napi::{buff_str::BuffStr, bytes_recv::JsBytes, fast_serde::FasterValue, fast_str::FastStr},
    response::InnerResp,
};

use super::{helpers::value_to_bytes, RequestBlob};

#[napi]
impl RequestBlob {
    #[inline(always)]
    #[napi(ts_args_type = "response: String")]
    /// This needs to be called at the end of every request even if nothing is returned
    /// This sent a raw text string to the client
    pub fn send_text(&mut self, response: BuffStr) -> Result<()> {
        let message = InnerResp::Text(response.0);
        self.send_result(message)
    }

    #[inline(always)]
    #[napi(ts_args_type = "response: String")]
    /// This needs to be called at the end of every request even if nothing is returned
    /// This sent a raw text string to the client
    /// This method will not check if a previous response has been sent doing so will result in undefined behavior but will be faster
    pub fn send_text_unchecked(&mut self, response: BuffStr) {
        let message = InnerResp::Text(response.0);
        let _ = self.send_result_checked(message, false);
    }

    #[inline(always)]
    #[napi(ts_args_type = "response: Buffer")]
    /// This needs to be called at the end of every request even if nothing is returned
    /// This sent a raw text string to the client but from a Javascript buffer, this can be faster
    pub fn send_bytes_text(&mut self, response: JsBytes) -> Result<()> {
        let message = InnerResp::Text(response.0);
        self.send_result(message)
    }

    #[inline(always)]
    #[napi(ts_args_type = "response: Buffer")]
    /// This needs to be called at the end of every request even if nothing is returned
    /// This sent a raw text string to the client but from a Javascript buffer, this can be faster
    /// This method will not check if a previous response has been sent doing so will result in undefined behavior but will be faster
    pub fn unchecked_send_bytes_text(&mut self, response: JsBytes) {
        let message = InnerResp::Text(response.0);
        let _ = self.send_result_checked(message, false);
    }

    #[inline(always)]
    #[napi]
    /// This needs to be called at the end of every request even if nothing is returned
    /// This will send an empty string to the user, useful for testing
    pub fn send_empty_text(&mut self) -> Result<()> {
        let message = InnerResp::EmptyString;
        self.send_result(message)
    }

    #[inline(always)]
    #[napi]
    /// This needs to be called at the end of every request even if nothing is returned
    /// This will send an empty string to the user, useful for testing
    /// This method will not check if a previous response has been sent doing so will result in undefined behavior but will be faster
    pub fn unchecked_send_empty_text(&mut self) {
        let message = InnerResp::EmptyString;
        let _ = self.send_result_checked(message, false);
    }

    #[inline(always)]
    #[napi(ts_args_type = "response: any")]
    /// This needs to be called at the end of every request even if nothing is returned
    /// This will send a JSON object to client, it will be serialized rust side so no need to stringify
    pub fn send_object(&mut self, response: Value) -> Result<()> {
        let message = InnerResp::Json(value_to_bytes(response)?);
        self.send_result(message)
    }

    #[inline(always)]
    #[napi(ts_args_type = "response: Object")]
    /// This needs to be called at the end of every request even if nothing is returned
    /// This will send a JSON object to client, it will be serialized rust side so no need to stringify
    /// This needs to be a key value object, any other is undefined behaviour
    pub fn send_fast_object(&mut self, response: FasterValue) -> Result<()> {
        let message = InnerResp::Json(value_to_bytes(response.0)?);
        self.send_result(message)
    }

    #[inline(always)]
    #[napi(ts_args_type = "response: Object")]
    /// This needs to be called at the end of every request even if nothing is returned
    /// This will send a JSON object to client, it will be serialized rust side so no need to stringify
    /// This needs to be a key value object, any other is undefined behaviour
    /// This method will not check if a previous response has been sent doing so will result in undefined behavior but will be faster
    /// The return value will only indicate if the message was sent or not
    pub fn send_fast_object_unchecked(&mut self, response: FasterValue) -> bool {
        let value = match value_to_bytes(response.0) {
            Ok(value) => value,
            Err(_) => return false,
        };

        let message = InnerResp::Json(value);
        self.send_result_checked(message, false).is_ok()
    }


    #[inline(always)]
    #[napi(ts_args_type = "response: String")]
    /// This needs to be called at the end of every request even if nothing is returned
    /// This will send a JSON object to client, however this is for objects that are already serialized to a string
    pub fn send_stringified_object(&mut self, response: BuffStr) -> Result<()> {
        let message = InnerResp::Json(response.0);
        self.send_result(message)
    }

    #[inline(always)]
    #[napi(ts_args_type = "group: String, file: String, context: String")]
    /// This needs to be called at the end of every request even if nothing is returned
    /// This will render a template and send it to the client
    /// The function takes in the group name which needs to be registerd earlier, the file name and the context object which includes the data to be rendered
    pub fn send_template_resp(
        &mut self,
        group_name: FastStr,
        file_name: FastStr,
        context_json: FastStr,
    ) -> Result<()> {
        let message = InnerResp::Template(group_name.0, file_name.0, context_json.0);
        self.send_result(message)
    }

    #[inline(always)]
    #[napi]
    /// This needs to be called at the end of every request even if nothing is returned
    /// This can be used to notify of a server error
    pub fn send_internal_server_error(&mut self) -> Result<()> {
        let message = InnerResp::ServerError;
        self.send_result(message)
    }

    #[inline(always)]
    #[napi]
    /// This needs to be called at the end of every request even if nothing is returned
    /// This can be used to notify of a server error with a message to display
    pub fn send_internal_server_error_with_message(&mut self, message: BuffStr) -> Result<()> {
        let message = InnerResp::ServerErrorWithMessage(message.0);
        self.send_result(message)
    }
}
