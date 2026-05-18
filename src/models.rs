#[derive(Debug, Clone)]
pub struct WifiNetwork {
    pub ssid: String,
    pub signal: u8,
    pub security: String,
    pub connected: bool,
    pub known: bool,
}

impl WifiNetwork {
    pub fn requires_password(&self) -> bool {
        let sec = self.security.trim().to_ascii_lowercase();
        !sec.is_empty() && sec != "open" && sec != "--"
    }
}
