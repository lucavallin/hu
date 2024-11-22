use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BridgeSettings {
    pub ip: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthSettings {
    pub devicetype: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub bridge: BridgeSettings,
    pub auth: AuthSettings,
}
