use serde::Serialize;

#[derive(Default, Serialize)]
pub struct User {
    pub uploaded_bytes: u64,
    pub downloaded_bytes: u64,
    pub ratio: f32,
    pub required_ratio: f32,
    pub perfect_flacs: Option<u32>,
    pub uploaded_count: u32,
    pub seeding: u32,
    pub leeching: u32,
    pub snatched: u32,
}

impl User {
    pub(crate) fn from_gazelle(user: gazelle_api::User) -> User {
        Self {
            uploaded_bytes: user.stats.uploaded,
            downloaded_bytes: user.stats.downloaded,
            ratio: user.stats.ratio,
            required_ratio: user.stats.required_ratio,
            perfect_flacs: Some(user.community.perfect_flacs),
            uploaded_count: user.community.uploaded,
            seeding: user.community.seeding,
            leeching: user.community.leeching,
            snatched: user.community.snatched,
        }
    }
}
