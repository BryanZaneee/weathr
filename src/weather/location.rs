use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct IpInfo {
    loc: String, // "34.0522,-118.2437"
}

pub async fn get_location_from_ip() -> Result<(f64, f64), Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .user_agent("weathr/1.0.0")
        .build()?;

    let resp = client
        .get("https://ipinfo.io/json")
        .send()
        .await?
        .json::<IpInfo>()
        .await?;

    let parts: Vec<&str> = resp.loc.split(',').collect();
    if parts.len() != 2 {
        return Err("Invalid location format from ipinfo.io".into());
    }

    let lat = parts[0].parse::<f64>()?;
    let lon = parts[1].parse::<f64>()?;

    Ok((lat, lon))
}
