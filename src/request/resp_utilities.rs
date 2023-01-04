use napi::bindgen_prelude::Uint8Array;

use crate::{
    napi::{buff_str::BuffStr, fast_str::FastStr, halfbrown::HalfBrown},
    router,
};

use super::{
    helpers::{convert_header_map, split_and_get_query_params},
    RequestBlob,
};

#[napi]
impl RequestBlob {
    #[inline(always)]
    #[napi]
    /// Add a new header to the response sent to the user
    pub fn add_header(&mut self, key: BuffStr, value: BuffStr) {
        if self.sent {
            return;
        }

        let headers = unsafe { self.headers.assume_init_mut() };

        if let Some(list_of_headers) = headers {
            list_of_headers.push((key.0, value.0))
        } else {
            *headers = Some(vec![(key.0, value.0)])
        }
    }

    #[inline(always)]
    #[napi]
    /// Set the returning status code for this response to the user
    /// Returns a boolean to indicate if the status code was set
    pub fn set_status_code(&mut self, status: u16) -> bool {
        if self.sent {
            return false;
        }

        // These are considered invalid status codes
        if !(100..1000).contains(&status) {
            return false;
        }

        self.status_code = Some(status);       
        true
    }

    #[inline(always)]
    #[napi]
    /// Get the query parameters as an object with each key and value
    /// this will only be null if an error has occurred
    pub fn get_query_params(&self) -> Option<HalfBrown<String, String>> {
        let query_string = self.get_data_val().uri().query()?.to_owned();
        Some(split_and_get_query_params(query_string))
    }

    #[inline(always)]
    #[napi]
    /// Get the url parameters as an object with each key and value
    /// this will only be null if an error has occurred
    pub fn get_url_params(&self) -> Option<HalfBrown<String, String>> {
        let method_str = self.get_data_val().method();
        router::read_only::get_params(self.get_data_val().path(), method_str.clone())
    }

    #[inline(always)]
    #[napi]
    /// Get the url parameters as an object with each key and value
    /// this will only be null if an error has occurred
    pub fn header_length(&self) -> i64 {
        let header_val = self.get_data_val().headers().len();

        header_val as i64
    }

    #[inline(always)]
    #[napi]
    /// Get the url parameters as an object with each key and value
    /// this will only be null if an error has occurred
    pub fn get_header(&self, name: FastStr) -> Option<String> {
        let header_val = self.get_data_val().headers().get(name.0)?;

        Some(header_val.to_str().ok()?.to_string())
    }

    #[inline(always)]
    #[napi]
    /// Get the url parameters as an object with each key and value
    /// this will only be null if an error has occurred
    pub fn get_all_headers(&self) -> HalfBrown<String, String> {
        let header_val = self.get_data_val().headers();
        convert_header_map(header_val)
    }

    #[inline(always)]
    #[napi]
    /// Retrieve the raw body bytes in a Uint8Array to be used
    pub fn get_body(&mut self) -> Uint8Array {
        match &self.body {
            Some(res) => (&res.clone() as &[u8]).into(),
            None => vec![].into(),
        }
    }
}
