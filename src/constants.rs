use std::sync::LazyLock;

use url::Url;

#[expect(
    clippy::expect_used,
    reason = "Hardcoded URL, validity guaranteed at compile time"
)]
pub static BASE_URL: LazyLock<Url> =
    LazyLock::new(|| Url::parse("https://api-rest-prod.incb.fr/api/").expect("Invalid Base URL"));

pub const USER_AGENT: &str = "MyTurboself/86 CFNetwork/3890.100.1 Darwin/27.0.0";
