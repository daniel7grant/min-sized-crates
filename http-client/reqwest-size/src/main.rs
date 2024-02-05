use reqwest::{get, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ipinfo {
    pub ip: String,
    pub hostname: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub loc: String,
    pub org: String,
    pub postal: String,
    pub timezone: String,
    pub readme: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let ipinfo: Ipinfo = get("https://ipinfo.io/json").await?.json().await?;

    println!("{}", ipinfo.ip);

    Ok(())
}
