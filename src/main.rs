use std::sync::atomic::{AtomicU64, Ordering};

use async_trait::async_trait;
use http::header::{CONTENT_LENGTH, CONTENT_TYPE};
use http::{Response, StatusCode};
use pingora::apps::http_app::ServeHttp;
use pingora::protocols::http::ServerSession;
use pingora::server::Server;
use pingora::services::listening::Service;

struct HelloApp {
    counter: AtomicU64,
}

#[async_trait]
impl ServeHttp for HelloApp {
    async fn response(&self, _server_session: &mut ServerSession) -> Response<Vec<u8>> {
        let count = self.counter.fetch_add(1, Ordering::SeqCst);
        let body = format!("Hello, World! あなたは{}人目の訪問者です。\n", count + 1);
        Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "text/plain")
            .header(CONTENT_LENGTH, body.len().to_string().as_str())
            .body(body.into_bytes())
            .unwrap()
    }
}

fn main() -> pingora::Result<()> {
    env_logger::init();
    let mut server = Server::new(None)?;
    server.bootstrap();

    let hello_app = HelloApp {
        counter: AtomicU64::new(0),
    };
    let mut hello_service = Service::new("hello_app".to_owned(), hello_app);
    hello_service.add_tcp("[::]:3000");
    server.add_service(hello_service);
    server.run_forever();
}