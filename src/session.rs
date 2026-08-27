use std::time::{Duration, SystemTime};

pub struct Session {
    pub access_token: String,
    pub expire_at: SystemTime,
}

impl Session {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            expire_at: SystemTime::now() + Duration::from_hours(5),
        }
    }

    pub fn is_expired(&self) -> bool {
        SystemTime::now() >= self.expire_at
    }
}
