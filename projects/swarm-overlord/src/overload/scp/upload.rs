use super::*;

impl SwarmOverlord {
    pub fn upload_task<C, P>(&self, content: C, remote_path: P) -> QResult<UploadTask>
    where
        P: AsRef<Path>,
        ContentResolver: TryFrom<C, Error = QError>,
    {
        let content = ContentResolver::try_from(content)?.content;
        Ok(UploadTask { content, target: remote_path.as_ref().to_path_buf(), permission: 0o644, session: &self.session })
    }
}

impl<'s> UploadTask<'s> {
    pub fn with_permission(mut self, permission: i32) -> Self {
        self.permission = permission;
        self
    }

    pub async fn execute(self) -> QResult<()> {
        let mut scp = self.session.scp_send(&self.target, self.permission, self.content.len() as u64, None)?;
        scp.write(&self.content)?;
        Ok(())
    }
}

impl TryFrom<&Path> for ContentResolver {
    type Error = QError;
    fn try_from(path: &Path) -> QResult<Self> {
        let mut file = File::open(path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        Ok(Self { content })
    }
}

impl TryFrom<&[u8]> for ContentResolver {
    type Error = QError;
    fn try_from(data: &[u8]) -> QResult<Self> {
        Ok(Self { content: data.to_vec() })
    }
}

impl TryFrom<&str> for ContentResolver {
    type Error = QError;
    fn try_from(data: &str) -> QResult<Self> {
        Ok(Self { content: data.as_bytes().to_vec() })
    }
}
