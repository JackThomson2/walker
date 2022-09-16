use bytes::Bytes;
use may_minihttp::Response;

#[derive(Clone)]
pub enum JsResponse {
    Text(Bytes),
    Json(Bytes),
    Raw(Bytes)
}

impl JsResponse {
    fn apply_headers(&self, rsp: &mut Response) {
        let message = match self {
            Self::Text(_) => "Content-Type: text/plain",
            Self::Json(_) => "Content-Type: application/json",
            Self::Raw(_) => "Content-Type: application/octet-stream",
        };

        rsp.header(message);
    }

    fn apply_response(&self, rsp: &mut Response) {
        match self {
            Self::Text(message) | Self::Json(message) => {

                let bytes = rsp.body_mut();
                bytes.extend_from_slice(message);
            },
            Self::Raw(data) => {
                let bytes = rsp.body_mut();
                bytes.extend_from_slice(data);
            }
        }
    }

    pub fn apply_to_response(&self, rsp: &mut Response) {
        self.apply_headers(rsp);
        self.apply_response(rsp);
    }
}