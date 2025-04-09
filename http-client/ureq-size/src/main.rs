use serde::{Deserialize, Serialize};
use ureq::{get, Error};

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

fn main() -> Result<(), Error> {
    let ipinfo: Ipinfo = get("https://ipinfo.io/json").call()?.body_mut().read_json()?;

    println!("{}", ipinfo.ip);

    Ok(())
}
