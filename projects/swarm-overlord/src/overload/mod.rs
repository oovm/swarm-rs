use std::sync::Arc;
use diagnostic_quick::QResult;

use futures::future::{Ready, ready};
use thrussh::{ChannelId, Error};
use thrussh::client::{Config, connect, Handler, Session};
use thrussh_keys::agent::client::AgentClient;
use thrussh_keys::key::{KeyPair, PublicKey};

/// Connect to the target computer using the ssh protocol
pub struct OverloadClient {}


impl Handler for OverloadClient {
    type Error = Error;
    type FutureBool = Ready<Result<(Self, bool), Error>>;
    type FutureUnit = Ready<Result<(Self, Session), Error>>;

    fn finished_bool(self, b: bool) -> Self::FutureBool {
        ready(Ok((self, b)))
    }
    fn finished(self, session: Session) -> Self::FutureUnit {
        ready(Ok((self, session)))
    }
    fn check_server_key(self, server_public_key: &PublicKey) -> Self::FutureBool {
        println!("check_server_key: {:?}", server_public_key);
        self.finished_bool(true)
    }
    fn channel_open_confirmation(self, channel: ChannelId, max_packet_size: u32, window_size: u32, session: Session) -> Self::FutureUnit {
        println!("channel_open_confirmation: {:?}", channel);
        self.finished(session)
    }
    fn data(self, channel: ChannelId, data: &[u8], session: Session) -> Self::FutureUnit {
        println!("data on channel {:?}: {:?}", channel, std::str::from_utf8(data));
        self.finished(session)
    }
}


#[tokio::test]
async fn main() -> QResult {
    let config = Config::default();
    let config = Arc::new(config);
    let sh = OverloadClient {};

    let key = KeyPair::generate_ed25519().unwrap();
    let mut agent = AgentClient::connect_env().await?;
    agent.add_identity(&key, &[]).await?;
    let mut session = connect(config, "localhost:22", sh).await?;
    if session.authenticate_future(std::env::var("USER").unwrap(), key.clone_public_key(), agent).await.1.unwrap() {
        let mut channel = session.channel_open_session().await.unwrap();
        channel.data(&b"Hello, world!"[..]).await.unwrap();
        if let Some(msg) = channel.wait().await {
            println!("{:?}", msg)
        }
    }
    Ok(())
}