use std::mem::MaybeUninit;
use actix_http::Request;
use bytes::Bytes;
use tokio::sync::oneshot::Sender;
use napi::Result;

use super::helpers::make_js_error;
use crate::response::{JsResponse, InnerResp};


#[napi]
pub struct RequestBlob {
    pub(crate) data: MaybeUninit<Request>,
    pub(crate) oneshot: MaybeUninit<Sender<JsResponse>>,
    pub(crate) sent: bool,
    pub(crate) body: Option<Bytes>,
    pub(crate) headers: MaybeUninit<Option<Vec<(Bytes, Bytes)>>>,
    pub(crate) written: usize,
    pub(crate) status_code: Option<u16>,
}

impl RequestBlob {
    pub fn new_empty_with_js() -> Box<Self> {
        Box::new(Self {
            data: MaybeUninit::uninit(),
            oneshot: MaybeUninit::uninit(),
            sent: false,
            body: None,
            headers: MaybeUninit::uninit(),
            written: 0,
            status_code: None,
        })
    }
    
    #[inline]
    pub fn store_self_data(&mut self, data: Request, sender: Sender<JsResponse>, body: Option<Bytes>) {
        let oneshot = MaybeUninit::new(sender);
        let headers = MaybeUninit::new(None);
        let data = MaybeUninit::new(data);

        if self.written > 0 {
            unsafe { self.data.assume_init_drop(); }
        }

        self.data = data;
        self.oneshot = oneshot;
        self.headers = headers;
        self.body = body;
        self.sent = false;
        self.written += 1;
        self.status_code = None;
    }

    #[inline(always)]
    pub(crate) fn get_data_val(&self) -> &Request {
        unsafe { self.data.assume_init_ref() }
    }

    #[inline(always)]
    pub fn send_result_checked(&mut self, inner: InnerResp, checked: bool) -> Result<()> {
        if checked && self.sent {
            return Err(make_js_error("Already sent response."));
        }

        self.sent = true;
        let oneshot = unsafe {
            let result = std::mem::replace(&mut self.oneshot, MaybeUninit::uninit());
            result.assume_init()
        };

        let headers = unsafe {
            let result = std::mem::replace(&mut self.headers, MaybeUninit::uninit());
            result.assume_init()
        };

        let js_resp = JsResponse { inner, headers, status_code: self.status_code };
        let res = oneshot.send(js_resp);

        if checked && res.is_err() {
            eprintln!("Error sending response, the reciever may have dropped.");
        }

        Ok(())
    }

    #[inline(always)]
    pub fn send_result(&mut self, response: InnerResp) -> Result<()> {
        self.send_result_checked(response, true)
    }
}