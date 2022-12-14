use super::*;

mod upload;

pub struct SendFileContent {
    content: Vec<u8>,
}

pub struct SendFileTask<'s> {
    content: Vec<u8>,
    target: PathBuf,
    permission: i32,
    session: &'s SSH2Session,
}
