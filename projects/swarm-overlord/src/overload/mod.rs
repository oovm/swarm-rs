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

pub mod github;
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
    pub fn shell_runner(&self) -> QResult<ShellRunner> {
        Ok(ShellRunner { session: &self.session })
    }
}

pub struct ShellRunner<'s> {
    session: &'s SSH2Session,
}

impl ShellRunner<'_> {
    pub async fn execute(&self, command: &str) -> QResult<String> {
        let mut shell = self.session.channel_session()?;
        shell.exec(command)?;
        let mut stdout = String::new();
        shell.read_to_string(&mut stdout)?;
        shell.wait_close()?;
        Ok(stdout)
    }
}
