use std::collections::HashMap;

use napi::{Error, Result, Status};

use crate::request::RequestBlob;

#[napi]
impl RequestBlob {
  #[napi]
  pub fn set_response(&self, response: String) -> Result<()> {
    self.oneshot.send(response).map_err(|_e| {
      Error::new(
        Status::GenericFailure,
        "Unable to send response".to_string(),
      )
    })
  }

  #[napi]
  pub fn get_params(&self) -> Option<HashMap<String, String>> {
    crate::router::store::get_params(self.data.path())
  }
}
