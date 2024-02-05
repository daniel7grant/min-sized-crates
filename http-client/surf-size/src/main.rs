use async_std::task;
use serde::{Deserialize, Serialize};
use surf::{get, Error};

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
    task::block_on(async {
        let ipinfo: Ipinfo = get("https://ipinfo.io/json").recv_json().await?;

        println!("{}", ipinfo.ip);

        Ok(())
    })
}
