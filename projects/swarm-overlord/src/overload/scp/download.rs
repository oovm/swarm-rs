use super::*;

impl SwarmOverlord {
    pub fn download_task<P>(&self, remote_path: P) -> QResult<DownloadTask>
    where
        P: AsRef<Path>,
    {
        Ok(DownloadTask { target: remote_path.as_ref().to_path_buf(), session: &self.session })
    }
}

impl<'s> DownloadTask<'s> {
    pub async fn execute(self) -> QResult<Vec<u8>> {
        // 下载文件
        let (mut scp, _) = self.session.scp_recv(&self.target)?;
        let mut buffer = Vec::new();
        scp.read_to_end(&mut buffer)?;
        // 关闭频道，等待全部内容传输完毕
        scp.send_eof()?;
        scp.wait_eof()?;
        scp.close()?;
        scp.wait_close()?;
        Ok(buffer)
    }
}
