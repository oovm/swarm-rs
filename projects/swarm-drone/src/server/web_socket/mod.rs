use super::*;

pub async fn ws_handler(ws: WebSocketUpgrade, user_agent: Option<TypedHeader<UserAgent>>) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected", user_agent.as_str());
    }
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    match socket.recv().await {
        Some(msg) => match msg {
            Ok(msg) => {
                let reply = match msg {
                    Message::Text(v) => socket.send(Message::Text(v)).await,
                    Message::Binary(v) => socket.send(Message::Pong(v)).await,
                    Message::Ping(v) => socket.send(Message::Pong(v)).await,
                    Message::Pong(_) => Ok(()),
                    Message::Close(_) => {
                        return;
                    }
                };
                if let Err(e) = reply {
                    println!("Error sending message: {}", e);
                }
            }
            Err(e) => {
                println!("client disconnected: {e}");
                return;
            }
        },
        None => {
            println!("client disconnected");
            return;
        }
    }
}
