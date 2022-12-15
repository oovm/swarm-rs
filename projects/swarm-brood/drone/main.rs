use std::{net::SocketAddr, str::FromStr};

use clap::Parser;

use swarm_drone::DroneWorker;

/// A simple CLI for the Swarm Drone
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct DroneBuilder {
    /// Default: `127.0.0.1:3000`
    #[arg(short, long)]
    socket: Option<String>,
    /// Config Path
    ///
    /// Default: `./swarm-drone.toml`
    #[arg(short, long)]
    config: Option<String>,
    ///
    #[arg(long, value_name = "ROUTER")]
    websocket: Option<String>,
    ///
    #[arg(long, value_name = "ROUTER")]
    restful: Option<String>,
    ///
    #[arg(long, value_name = "ROUTER")]
    graphql: Option<String>,
}

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "example_websockets=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();
    let builder = DroneBuilder::parse();
    let mut worker = DroneWorker::default();
    let socket = match &builder.socket {
        Some(s) => SocketAddr::from_str(s),
        None => SocketAddr::from_str("127.0.0.1"),
    };
    match socket {
        Ok(o) => {
            worker.socket = o;
        }
        Err(_) => {}
    }
    worker.serve().await.unwrap()
}
