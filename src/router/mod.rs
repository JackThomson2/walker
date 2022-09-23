use matchit::Router;

use crate::types::CallBackFunction;

pub mod node_functions;
pub mod read_only;
pub mod store;


pub type ReaderLookup = Router<CallBackFunction>;