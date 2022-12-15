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
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

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
        let app = self.make_router().into_make_service();
        let server = Server::try_bind(&self.socket)?.serve(app);
        server.await
    }
    fn make_router(&self) -> Router {
        let mut out = Router::new();
        if let Some(path) = &self.websocket {
            out = out.route(path, get(ws_handler))
        }
        let tracer = TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true));
        out.layer(tracer)
    }
}

async fn ws_handler(ws: WebSocketUpgrade, user_agent: Option<TypedHeader<UserAgent>>) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        tracing::info!("`{}` connected", user_agent.as_str());
    }
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    if let Some(msg) = socket.recv().await {
        match msg {
            Ok(msg) => {
                tracing::error!("Client says: {:?}", msg);
                if socket.send(Message::Text(format!("{:?}", msg))).await.is_err() {
                    tracing::error!("client disconnected");
                    return;
                }
            }
            Err(e) => {
                tracing::error!("client disconnected: {e}");
                return;
            }
        }
    }
}
