use std::env;

use serde::Deserialize;

const DEFAULT_URL: &str = "https://ip.shnitzel.org";

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpInfo {
    pub ip: String,
    pub city: String,
    pub region: String,
    pub postal_code: String,
    pub country: String,
    pub isp: String,
    pub asn: u32,
    pub timezone: String,
    pub latitude: f64,
    pub longitude: f64,
}

pub fn fetch_ip_info() -> color_eyre::Result<IpInfo> {
    let url = env::var("IPMOG_URL").unwrap_or_else(|_| DEFAULT_URL.to_string());
    let resp = ureq::get(&url).call()?;
    let body = resp.into_string()?;
    let info: IpInfo = serde_json::from_str(&body)?;
    Ok(info)
}
