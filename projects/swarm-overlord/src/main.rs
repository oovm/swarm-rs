
#[tokio::main]
async fn main() {
    let config = thrussh::client::Config::default();
    let config = Arc::new(config);
    let sh = Client {};

    let key = thrussh_keys::key::KeyPair::generate_ed25519().unwrap();
    let mut agent = thrussh_keys::agent::client::AgentClient::connect_env().await.unwrap();
    agent.add_identity(&key, &[]).await.unwrap();
    let mut session = thrussh::client::connect(config, "localhost:22", sh).await.unwrap();
    if session.authenticate_future(std::env::var("USER").unwrap(), key.clone_public_key(), agent).await.1.unwrap() {
        let mut channel = session.channel_open_session().await.unwrap();
        channel.data(&b"Hello, world!"[..]).await.unwrap();
        if let Some(msg) = channel.wait().await {
            println!("{:?}", msg)
        }
    }
}