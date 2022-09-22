use bytes::BytesMut;
use napi::bindgen_prelude::Buffer;

use std::io;
use std::mem::MaybeUninit;

pub struct Response<'a> {
    headers: [&'static str; 16],
    headers_len: usize,
    status_message: StatusMessage,
    rsp_buf: &'a mut BytesMut,
}

struct StatusMessage {
    code: &'static str,
    msg: &'static str,
}

impl<'a> Response<'a> {
    pub(crate) fn new(rsp_buf: &'a mut BytesMut) -> Response {
        let headers: [&'static str; 16] = unsafe {
            let h: [MaybeUninit<&'static str>; 16] = MaybeUninit::uninit().assume_init();
            std::mem::transmute(h)
        };

        Response {
            headers,
            headers_len: 0,
            status_message: StatusMessage {
                code: "200",
                msg: "Ok",
            },
            rsp_buf,
        }
    }

    pub fn status_code(&mut self, code: &'static str, msg: &'static str) -> &mut Self {
        self.status_message = StatusMessage { code, msg };
        self
    }

    pub fn header(&mut self, header: &'static str) -> &mut Self {
        debug_assert!(self.headers_len < 16);
        *unsafe { self.headers.get_unchecked_mut(self.headers_len) } = header;
        self.headers_len += 1;
        self
    }

    #[inline]
    pub fn write_raw_bytes(&mut self, buf: &[u8]) {
        self.rsp_buf.extend_from_slice(buf)
    }

    #[inline]
    pub fn write_bytes(&mut self, buf: &bytes::Bytes) {
        self.rsp_buf.extend_from_slice(buf)
    }

    #[inline]
    pub fn write_buffer(&mut self, buf: &Buffer) {
        self.rsp_buf.extend_from_slice(buf)
    }

    #[inline]
    fn body_len(&self) -> usize {
        self.rsp_buf.len()
    }

    #[inline]
    fn get_body(&mut self) -> &[u8] {
        self.rsp_buf.as_ref()
    }

    #[inline]
    fn clear_body(&mut self) {
        self.rsp_buf.clear()
    }
}

pub fn encode(mut msg: Response, buf: &mut BytesMut) {
    if msg.status_message.msg == "Ok" {
        buf.extend_from_slice(b"HTTP/1.1 200 Ok\r\nServer: walker\r\nDate: ");
    } else {
        buf.extend_from_slice(b"HTTP/1.1 ");
        buf.extend_from_slice(msg.status_message.code.as_bytes());
        buf.extend_from_slice(b" ");
        buf.extend_from_slice(msg.status_message.msg.as_bytes());
        buf.extend_from_slice(b"\r\nServer: walker\r\nDate: ");
    }
    crate::minihttp::date::set_date(buf);
    buf.extend_from_slice(b"\r\nContent-Length: ");
    let mut length = itoa::Buffer::new();
    buf.extend_from_slice(length.format(msg.body_len()).as_bytes());

    for i in 0..msg.headers_len {
        let h = *unsafe { msg.headers.get_unchecked(i) };
        buf.extend_from_slice(b"\r\n");
        buf.extend_from_slice(h.as_bytes());
    }

    buf.extend_from_slice(b"\r\n\r\n");
    buf.extend_from_slice(msg.get_body());
    msg.clear_body();
}

// impl io::Write for the response body
pub struct BodyWriter<'a>(pub &'a mut BytesMut);

impl<'a> io::Write for BodyWriter<'a> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
