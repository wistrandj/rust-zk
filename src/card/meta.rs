use super::Face;
use std::ops::Deref;
use std::cmp::Ordering;

pub struct Meta {
    pub face: Face,
    pub create_time: Timestamp,
    pub modify_time: Timestamp,
    pub content_sha256: String,
    pub commit_user: String,
    pub commit_email: String,
}

impl Deref for Meta {
    type Target = Face;
    fn deref(&self) -> &Face {
        &self.face
    }
}


// Note(wistrandj): Later, add the create and modify datetimes in the
// sqlite3 timeline file. Until now, use an dummy implementation.
pub struct Timestamp { }
impl PartialEq for Timestamp {
    fn eq(&self, other: &Timestamp) -> bool {
        // Note(wistrandj): Everything are the same!
        return true;
    }
}

impl Eq for Timestamp {}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Timestamp) -> Option<Ordering> {
        return Some(Ordering::Equal);
    }
}


