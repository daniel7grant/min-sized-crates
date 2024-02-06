use curl::easy::Easy;
use serde::Deserialize;
use serde::Serialize;

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

fn main() -> Result<(), curl::Error> {
    let mut curl = Easy::new();

    curl.url("http://ipinfo.io/json")?;
    curl.write_function(|s| {
        let ipinfo: Ipinfo = serde_json::from_slice(s).unwrap();
        println!("{}", ipinfo.ip);
        Ok(s.len())
    })?;
    curl.perform()?;

    Ok(())
}
