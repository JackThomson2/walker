use tokio::sync::oneshot::Sender;

pub struct HttpRequest {
    pub route: String,
    pub response: Sender<bytes::Bytes>
}