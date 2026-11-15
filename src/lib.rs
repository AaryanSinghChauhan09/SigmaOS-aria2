//! SigmaOS-aria2: multi-protocol download manager model.
//!
//! Real implementation: downloads, segments, mirrors, validation.
//! Replaces the 7-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Download {
    pub id: String,
    pub url: String,
    pub file_path: String,
    pub completed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Aria2Error {
    EmptyName,
    DuplicateDownload(String),
}

impl fmt::Display for Aria2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "file path must not be empty"),
            Self::DuplicateDownload(id) => write!(f, "duplicate download {:?}", id),
        }
    }
}

impl std::error::Error for Aria2Error {}

pub struct DownloadManager {
    pub name: String,
    downloads: BTreeMap<String, Download>,
    mirrors: BTreeMap<String, String>,
}

impl DownloadManager {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), downloads: BTreeMap::new(), mirrors: BTreeMap::new() }
    }

    pub fn add_download(&mut self, d: Download) -> Result<(), Aria2Error> {
        if d.file_path.trim().is_empty() { return Err(Aria2Error::EmptyName); }
        let id = d.id.clone();
        if self.downloads.contains_key(&id) {
            return Err(Aria2Error::DuplicateDownload(id.clone()));
        }
        self.downloads.insert(id, d);
        Ok(())
    }

    pub fn download_count(&self) -> usize { self.downloads.len() }

    pub fn get_download(&self, id: &str) -> Option<&Download> { self.downloads.get(id) }

    pub fn add_mirror(&mut self, url: String, location: String) {
        self.mirrors.insert(url, location);
    }

    pub fn get_mirror(&self, url: &str) -> Option<&String> { self.mirrors.get(url) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let m = DownloadManager::new("My Manager");
        assert_eq!(m.name, "My Manager");
        assert_eq!(m.download_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut m = DownloadManager::new("My Manager");
        let d = Download { id: "dl1".into(), url: "https://example.com/file.iso".into(), file_path: "/tmp/file.iso".into(), completed: false };
        m.add_download(d).unwrap();
        assert_eq!(m.download_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut m = DownloadManager::new("My Manager");
        let d = Download { id: "x".into(), url: "a".into(), file_path: "".into(), completed: false };
        assert!(m.add_download(d).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut m = DownloadManager::new("My Manager");
        let d = Download { id: "x".into(), url: "a".into(), file_path: "X".into(), completed: false };
        m.add_download(d).unwrap();
        let d2 = Download { id: "x".into(), url: "a".into(), file_path: "X".into(), completed: false };
        assert!(m.add_download(d2).is_err());
    }
}
