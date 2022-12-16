use diagnostic_quick::error_3rd::Url;
pub mod github;
pub mod local;

/// Metadata about a build artifact.
pub struct SwarmArtifact {
    pub name: String,
    pub version: String,
    pub path: String,
    pub hash: String,
    pub size: u64,
    pub mime: String,
}

pub trait ArtifactAddress {
    fn download_link(&self) -> Url;
}
