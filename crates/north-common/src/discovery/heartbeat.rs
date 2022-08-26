#[derive(Default, Clone, Debug)]
pub struct HeartbeatOptions {
    pub enabled: bool,

    pub ttl_in_seconds: Option<i64>,
}
