use std::time::{Duration, SystemTime};

pub struct HttpManager {
    access_token: String,
    expire_at: SystemTime,
}

impl HttpManager {
    pub fn update_access_token(&mut self, token: String) {
        self.access_token = token;
        self.expire_at = SystemTime::now() + Duration::from_hours(5);
    }

    pub fn is_token_expired(&self) -> bool {
        SystemTime::now() >= self.expire_at
    }
}
