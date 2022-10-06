use crate::{request::RequestBlob, napi::tsfn::ThreadsafeFunction};

pub type CallBackFunction = ThreadsafeFunction<RequestBlob>;
