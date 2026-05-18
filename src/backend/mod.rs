pub mod nmcli;

use crate::models::WifiNetwork;

pub trait WifiBackend {
    fn scan_networks(&self) -> anyhow::Result<Vec<WifiNetwork>>;
    fn connect_known(&self, ssid: &str) -> anyhow::Result<()>;
    fn connect_with_password(&self, ssid: &str, password: &str) -> anyhow::Result<()>;
}
