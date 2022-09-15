use std::collections::HashMap;

use napi::{bindgen_prelude::Buffer, Error, Result, Status};

use crate::{request::RequestBlob, Methods};

#[napi]
impl RequestBlob {
  #[inline]
  #[napi]
  /// This needs to be called at the end of every request even if nothing is returned
  pub fn set_response(&self, response: String) -> Result<()> {
    self.oneshot.send(response).map_err(|_e| {
      Error::new(
        Status::GenericFailure,
        "Unable to send response".to_string(),
      )
    })
  }

  // #[inline]
  // #[napi]
  // /// Get the url parameters as an object with each key and value
  // /// this will only be null if an error has occurred
  // pub fn get_params(&self) -> Option<HashMap<String, String>> {
  //   let method_str = self.data.method().to_uppercase();
  //   let method = match Methods::from_str(&method_str) {
  //     Some(res) => res,
  //     None => {
  //       return None;
  //     }
  //   };

  //   crate::router::store::get_params(self.data.path(), method)
  // }

  #[napi]
  /// Retrieve the raw body bytes in a Uint8Array to be used
  pub fn get_body(&self) -> Buffer {
    self.data.body().into()
  }
}
