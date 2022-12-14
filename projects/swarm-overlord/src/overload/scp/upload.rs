use super::*;

impl SwarmOverlord {
    pub fn upload_task<C, P>(&self, content: C, path: P) -> QResult<SendFileTask>
    where
        P: AsRef<Path>,
        SendFileContent: TryFrom<C, Error = QError>,
    {
        let content = SendFileContent::try_from(content)?.content;
        Ok(SendFileTask { content, target: path.as_ref().to_path_buf(), permission: 0o644, session: &self.session })
    }
}

impl<'s> SendFileTask<'s> {
    pub fn with_permission(mut self, permission: i32) -> Self {
        self.permission = permission;
        self
    }

    pub async fn send(self) -> QResult<()> {
        let mut scp = self.session.scp_send(&self.target, self.permission, self.content.len() as u64, None)?;
        scp.write(&self.content)?;
        Ok(())
    }
}

impl TryFrom<&Path> for SendFileContent {
    type Error = QError;
    fn try_from(path: &Path) -> QResult<Self> {
        let mut file = File::open(path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        Ok(Self { content })
    }
}

impl TryFrom<&[u8]> for SendFileContent {
    type Error = QError;
    fn try_from(data: &[u8]) -> QResult<Self> {
        Ok(Self { content: data.to_vec() })
    }
}

impl TryFrom<&str> for SendFileContent {
    type Error = QError;
    fn try_from(data: &str) -> QResult<Self> {
        Ok(Self { content: data.as_bytes().to_vec() })
    }
}
