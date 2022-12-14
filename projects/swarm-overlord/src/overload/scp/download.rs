use super::*;

impl SwarmOverlord {
    pub fn download_task<P>(&self, remote_path: P) -> QResult<DownloadTask>
        where
            P: AsRef<Path>,
    {

        let (mut scp, _) = self.session.scp_recv(self.)?;
        let mut buffer = Vec::new();
        scp.read_to_end(&mut buffer)?;

        let content = ContentResolver::try_from(content)?.content;
        Ok(UploadTask { content, target: remote_path.as_ref().to_path_buf(), permission: 0o644, session: &self.session })
    }
}

impl<'s> DownloadTask<'s> {
    pub async fn execute(self) -> QResult<Vec<u8>> {
        // 下载文件
        let (mut remote_file, _) = self.session.scp_recv(&self.target)?;
        let mut buffer = Vec::new();
        remote_file.read_to_end(&mut buffer).unwrap();

        // 关闭频道，等待全部内容传输完毕
        remote_file.send_eof()?;
        remote_file.wait_eof()?;
        remote_file.close()?;
        remote_file.wait_close()?;
        Ok            (buffer)
    }

}
