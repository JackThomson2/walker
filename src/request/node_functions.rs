use crate::request::RequestBlob;

#[napi]
impl RequestBlob {
    #[napi]
    pub fn get_message(&self) -> String {
        self.message.to_string()
    }

    #[napi]
    pub fn set_response(&self, response: String) {
        self.oneshot.send(response).unwrap()
    }
}