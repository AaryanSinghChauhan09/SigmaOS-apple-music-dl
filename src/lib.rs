//! SigmaOS-apple-music-dl: Apple Music downloader model.
//!
//! Real implementation: tracks, downloads, validation.
//! Replaces the 31-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Download {
    pub id: String,
    pub track_id: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppleMusicDlError {
    EmptyTrackName,
    DuplicateTrack(String),
}

impl fmt::Display for AppleMusicDlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTrackName => write!(f, "track name must not be empty"),
            Self::DuplicateTrack(id) => write!(f, "duplicate track {:?}", id),
        }
    }
}

impl std::error::Error for AppleMusicDlError {}

pub struct AppleMusicDl {
    pub name: String,
    tracks: BTreeMap<String, Track>,
    downloads: BTreeMap<String, Download>,
}

impl AppleMusicDl {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), tracks: BTreeMap::new(), downloads: BTreeMap::new() }
    }

    pub fn add_track(&mut self, t: Track) -> Result<(), AppleMusicDlError> {
        if t.name.trim().is_empty() { return Err(AppleMusicDlError::EmptyTrackName); }
        let id = t.id.clone();
        if self.tracks.contains_key(&id) {
            return Err(AppleMusicDlError::DuplicateTrack(id.clone()));
        }
        self.tracks.insert(id, t);
        Ok(())
    }

    pub fn track_count(&self) -> usize { self.tracks.len() }

    pub fn get_track(&self, id: &str) -> Option<&Track> { self.tracks.get(id) }

    pub fn add_download(&mut self, d: Download) {
        self.downloads.insert(d.id.clone(), d);
    }

    pub fn get_download(&self, id: &str) -> Option<&Download> { self.downloads.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let a = AppleMusicDl::new("AppleMusicDl");
        assert_eq!(a.name, "AppleMusicDl");
        assert_eq!(a.track_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut a = AppleMusicDl::new("AppleMusicDl");
        let track = Track { id: "t1".into(), name: "Song".into(), url: "https://music.apple.com/s".into() };
        a.add_track(track).unwrap();
        assert_eq!(a.track_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut a = AppleMusicDl::new("AppleMusicDl");
        let track = Track { id: "x".into(), name: "".into(), url: "x".into() };
        assert!(a.add_track(track).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = AppleMusicDl::new("AppleMusicDl");
        let track = Track { id: "x".into(), name: "X".into(), url: "x".into() };
        a.add_track(track).unwrap();
        let track2 = Track { id: "x".into(), name: "X".into(), url: "x".into() };
        assert!(a.add_track(track2).is_err());
    }
}
