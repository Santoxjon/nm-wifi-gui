use std::collections::HashSet;
use std::process::Command;

use anyhow::{anyhow, Context};

use crate::backend::WifiBackend;
use crate::models::WifiNetwork;

#[derive(Debug, Clone, Default)]
pub struct NmcliBackend;

impl NmcliBackend {
    fn split_escaped_fields(line: &str, delimiter: char, max_fields: usize) -> Vec<String> {
        let mut fields = Vec::with_capacity(max_fields);
        let mut current = String::new();
        let mut chars = line.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                if let Some(next) = chars.next() {
                    current.push(next);
                }
                continue;
            }

            if ch == delimiter && fields.len() + 1 < max_fields {
                fields.push(current);
                current = String::new();
            } else {
                current.push(ch);
            }
        }

        fields.push(current);

        while fields.len() < max_fields {
            fields.push(String::new());
        }

        fields
    }

    fn run_nmcli(args: &[&str]) -> anyhow::Result<String> {
        let output = Command::new("nmcli")
            .args(args)
            .output()
            .with_context(|| format!("Failed to execute nmcli with args: {args:?}"))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(anyhow!(
                "nmcli failed (status {}): {}",
                output.status,
                if stderr.is_empty() {
                    "Unknown error".to_string()
                } else {
                    stderr
                }
            ))
        }
    }

    fn known_networks(&self) -> anyhow::Result<HashSet<String>> {
        let out = Self::run_nmcli(&["-t", "-f", "NAME,TYPE", "connection", "show"])?;

        let mut known = HashSet::new();
        for line in out.lines().filter(|l| !l.trim().is_empty()) {
            let parts = Self::split_escaped_fields(line, ':', 2);
            let name = parts[0].trim();
            let conn_type = parts[1].trim();

            if conn_type == "802-11-wireless" && !name.is_empty() {
                known.insert(name.to_string());
            }
        }

        Ok(known)
    }
}

impl WifiBackend for NmcliBackend {
    fn scan_networks(&self) -> anyhow::Result<Vec<WifiNetwork>> {
        let known = self.known_networks().unwrap_or_default();
        let out = Self::run_nmcli(&[
            "-t",
            "-f",
            "IN-USE,SSID,SIGNAL,SECURITY",
            "dev",
            "wifi",
            "list",
            "--rescan",
            "yes",
        ])?;

        let mut networks = Vec::new();

        for line in out.lines().filter(|l| !l.trim().is_empty()) {
            let parts = Self::split_escaped_fields(line, ':', 4);
            let in_use = parts[0].trim();
            let ssid = parts[1].trim();
            let signal_raw = parts[2].trim();
            let security_raw = parts[3].trim();

            if ssid.is_empty() {
                continue;
            }

            let signal = signal_raw.parse::<u8>().unwrap_or(0);
            let connected = in_use.contains('*');
            let security = if security_raw.is_empty() || security_raw == "--" {
                "open".to_string()
            } else {
                security_raw.to_string()
            };

            networks.push(WifiNetwork {
                ssid: ssid.to_string(),
                signal,
                security,
                connected,
                known: known.contains(ssid),
            });
        }

        networks.sort_by(|a, b| {
            b.connected
                .cmp(&a.connected)
                .then(b.signal.cmp(&a.signal))
                .then(a.ssid.cmp(&b.ssid))
        });

        Ok(networks)
    }

    fn connect_known(&self, ssid: &str) -> anyhow::Result<()> {
        Self::run_nmcli(&["dev", "wifi", "connect", ssid]).map(|_| ())
    }

    fn connect_with_password(&self, ssid: &str, password: &str) -> anyhow::Result<()> {
        Self::run_nmcli(&["dev", "wifi", "connect", ssid, "password", password]).map(|_| ())
    }
}
