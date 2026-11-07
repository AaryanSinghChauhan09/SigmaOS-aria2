//! aria2 model — lightweight download utility.

#[derive(Debug, Clone)]
pub struct Download {
    pub uri: String,
    pub dir: String,
    pub status: String,
    pub progress: f64,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub downloads: Vec<Download>,
}

impl Default for Model {
    fn default() -> Self { Self { downloads: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, uri: &str, dir: &str) -> bool {
        self.downloads.push(Download { uri: uri.into(), dir: dir.into(), status: "queued".into(), progress: 0.0 });
        true
    }

    pub fn start(&mut self, index: usize) -> bool {
        if let Some(d) = self.downloads.get_mut(index) { d.status = "downloading".into(); true } else { false }
    }

    pub fn list(&self) -> &[Download] { &self.downloads }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_start() {
        let mut a = Model::new();
        assert!(a.add("https://example.com/file", "/downloads"));
        assert!(a.start(0));
        assert_eq!(a.list()[0].status, "downloading");
    }
}
