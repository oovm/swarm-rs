use std::net::SocketAddr;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    response::IntoResponse,
    routing::get,
    Router, Server,
};
use headers::UserAgent;
// use tokio_cron_scheduler::JobScheduler;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use self::web_socket::ws_handler;

mod web_socket;

pub struct DroneWorker {
    pub socket: SocketAddr,
    websocket: Option<String>,
    restful: Option<String>,
    graphql: Option<String>,
}

impl Default for DroneWorker {
    fn default() -> Self {
        Self {
            socket: SocketAddr::from(([127, 0, 0, 1], 3000)),
            websocket: Some("/".to_string()),
            restful: None,
            graphql: None,
        }
    }
}

impl DroneWorker {
    pub fn with_socket(mut self, socket: SocketAddr) -> Self {
        self.socket = socket;
        self
    }
    pub fn with_port(mut self, port: u16) -> Self {
        self.socket.set_port(port);
        self
    }
    pub fn with_ipv4(mut self, ip: [u8; 4]) -> Self {
        self.socket.set_ip(ip.into());
        self
    }
    pub fn with_ipv6(mut self, ip: [u8; 16]) -> Self {
        self.socket.set_ip(ip.into());
        self
    }
    pub fn with_websocket(mut self, path: &str) -> Self {
        self.websocket = Some(path.to_string());
        self
    }
    pub fn with_restful(mut self, path: &str) -> Self {
        self.restful = Some(path.to_string());
        self
    }
    pub fn with_graphql(mut self, path: &str) -> Self {
        self.graphql = Some(path.to_string());
        self
    }
}

impl DroneWorker {
    pub async fn serve(self) -> Result<(), hyper::Error> {
        let router = self.make_router();
        // let jobs = JobScheduler::new().await.unwrap();
        let tracer = TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true));

        let app = router.layer(tracer).into_make_service();

        let server = Server::try_bind(&self.socket)?.serve(app);

        server.await
    }
    fn make_router(&self) -> Router {
        let mut out = Router::new();
        if let Some(path) = &self.websocket {
            out = out.route(path, get(ws_handler))
        }
        out
    }
}
