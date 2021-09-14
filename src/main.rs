use std::path::PathBuf;
use structopt::StructOpt;
use serde::{Deserialize, Serialize};
use reqwest::Result;
use crate::config::Config;


mod config;

#[derive(StructOpt)]
struct Opt {
    #[structopt(
    short,
    long,
    help = "config file to use",
    default_value = "./config.toml"
    )]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();
    let config = std::fs::read(&opt.config).expect("Config file reading error");
    let config = toml::from_slice::<Config>(&config).expect("Config file parsing error");

    let client = reqwest::ClientBuilder::new().user_agent("curl").build()?;
    let res: IPInfo = client.get("https://api.myip.com/").send().await?.json().await?;
    let req = DNSRequest::new(config.domain, res.ip);
    let url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", config.zone, config.entry);
    client.put(url).bearer_auth(config.token).json(&req).send().await?;

    Ok(())

}

#[derive(Deserialize)]
struct IPInfo {
    ip: String,
}

#[derive(Serialize)]
struct DNSRequest {
    #[serde(rename = "type")]
    dtype: String,
    name: String,
    content: String,
    ttl: u32,
    proxied: bool,
}

impl DNSRequest {

    fn new(name: String, content: String) -> Self {
        DNSRequest {
            dtype: "A".to_string(),
            name,
            content,
            ttl: 1,
            proxied: true
        }
    }

}

