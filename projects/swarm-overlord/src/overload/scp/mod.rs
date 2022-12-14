use super::*;

mod download;
mod upload;

pub struct ContentResolver {
    content: Vec<u8>,
}

pub struct UploadTask<'s> {
    content: Vec<u8>,
    target: PathBuf,
    permission: i32,
    session: &'s SSH2Session,
}

pub struct DownloadTask<'s> {
    target: PathBuf,
    session: &'s SSH2Session,
}
