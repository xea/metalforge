use std::time::Duration;
use crate::song::key::Key;

#[derive(Clone)]
pub struct Metadata {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub year: u16,
    pub length: Duration,
    pub key: Option<Key>
}