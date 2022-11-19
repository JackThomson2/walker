use std::mem::MaybeUninit;
use ntex::http::Request;
use ntex::util::Bytes;
use napi::Result;
use kanal::{AsyncReceiver, Sender};

use super::helpers::make_js_error;
use crate::response::{JsResponse, InnerResp};

#[napi]
pub struct RequestBlob {
    pub(crate) data: MaybeUninit<Request>,
    pub(crate) reciever: AsyncReceiver<JsResponse>,
    pub(crate) sender: Sender<JsResponse>,
    pub(crate) sent: bool,
    pub(crate) body: Option<Bytes>,
    pub(crate) headers: MaybeUninit<Option<Vec<(Bytes, Bytes)>>>,
    pub(crate) written: usize,
    pub(crate) status_code: Option<u16>,
}

impl RequestBlob {
    pub fn new_empty_with_js() -> Box<Self> {
        let (send, recv) = kanal::bounded(0);

        let recv = {
            let copied = recv.clone_async();
            drop(recv);

            copied
        };

        Box::new(Self {
            data: MaybeUninit::uninit(),
            reciever: recv,
            sender: send,
            sent: false,
            body: None,
            headers: MaybeUninit::uninit(),
            written: 0,
            status_code: None,
        })
    }
    
    #[inline]
    pub fn store_self_data(&mut self, data: Request, body: Option<Bytes>) {
        let headers = MaybeUninit::new(None);
        let data = MaybeUninit::new(data);

        if self.written > 0 {
            unsafe { self.data.assume_init_drop(); }
        }

        self.data = data;
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

        let headers = unsafe {
            let result = std::mem::replace(&mut self.headers, MaybeUninit::uninit());
            result.assume_init()
        };

        let js_resp = JsResponse { inner, headers, status_code: self.status_code };
        let res = self.sender.send(js_resp);

        if checked && res.is_err() {
            return Err(make_js_error("Error with sending the response."))
        }

        Ok(())
    }

    #[inline(always)]
    pub fn send_result(&mut self, response: InnerResp) -> Result<()> {
        self.send_result_checked(response, true)
    }
}
