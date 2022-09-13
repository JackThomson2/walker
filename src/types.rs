use napi::{threadsafe_function::{ThreadsafeFunction, ErrorStrategy}};

use crate::request::RequestBlob;

pub type CallBackFunction = ThreadsafeFunction<Vec<RequestBlob>, ErrorStrategy::Fatal>;
