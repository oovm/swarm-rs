use swarm_drone::DroneWorker;

pub struct DroneBuilder {
    socket: String,
}

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "example_websockets=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();
    let worker = DroneWorker::default().serve().await.unwrap();
}
