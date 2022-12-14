use fs::File;
use std::{
    fs,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    path::{Path, PathBuf},
};

use diagnostic_quick::{error_3rd::SSH2Session, QError, QResult};

pub struct SwarmOverlord {
    session: SSH2Session,
}

pub mod scp;

impl SwarmOverlord {
    pub async fn login_password<A>(address: A, user: &str, password: &str) -> QResult<Self>
    where
        A: ToSocketAddrs,
    {
        let tcp = TcpStream::connect(address)?;
        let mut session = SSH2Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;
        session.userauth_password(user, password)?;
        if !session.authenticated() {
            Err("Authentication failed")?
        }
        Ok(Self { session })
    }
}

#[tokio::test]
async fn main() -> QResult {
    let client = SwarmOverlord::login_password("192.168.1.100:22", "root", "projecta").await?;
    let session = &client.session;

    let mut channel = session.channel_session()?;

    // 执行命令并打印输出
    channel.exec("ls").unwrap();
    let mut ls = String::new();
    channel.read_to_string(&mut ls).unwrap();
    println!("{}", ls);
    channel.wait_close().unwrap();

    // 上传文件
    let data: &[u8] = include_bytes!("../../Cargo.toml");
    client.upload_task(data, "/tmp/Cargo.toml")?.send().await?;

    // 下载文件
    let (mut remote_file, _) = session.scp_recv(Path::new("Cargo.toml")).unwrap();
    let mut read = Vec::new();
    remote_file.read_to_end(&mut read).unwrap();

    // 关闭频道，等待全部内容传输完毕
    remote_file.send_eof().unwrap();
    remote_file.wait_eof().unwrap();
    remote_file.close().unwrap();
    remote_file.wait_close().unwrap();
    Ok(())
}
