use std::{
    fs,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    path::Path,
};

use diagnostic_quick::QResult;
use ssh2::Session;

pub struct OverloadClient {
    session: Session,
}

impl OverloadClient {
    pub fn login_password<A>(address: A, user: &str, password: &str) -> QResult<Self>
    where
        A: ToSocketAddrs,
    {
        let tcp = TcpStream::connect(address)?;
        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;
        session.userauth_password(user, password)?;
        if session.authenticated() {
            Ok(Self { session })
        }
        else {
            return Err("Authentication failed".into());
        }
    }
}

impl Drop for OverloadClient {
    fn drop(&mut self) {
        todo!()
    }
}

#[tokio::test]
async fn main() -> QResult {
    OverloadClient::login_password("192.168.1.100:22", "root", "projecta")?;

    let tcp = TcpStream::connect("192.168.1.100:22")?;
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();

    session.userauth_password("root", "projecta").unwrap();
    session.authenticated();

    let mut channel = session.channel_session().unwrap();

    // 执行命令并打印输出
    channel.exec("ls").unwrap();
    let mut ls = String::new();
    channel.read_to_string(&mut ls).unwrap();
    println!("{}", ls);
    channel.wait_close().unwrap();

    // 上传文件
    let result = fs::read("Cargo.toml").unwrap();
    let mut remote_file = session.scp_send(Path::new("Cargo.toml"), 0o755, result.len() as u64, None).unwrap();
    remote_file.write(&result).unwrap();

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
