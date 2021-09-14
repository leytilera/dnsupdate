use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub zone: String,
    pub entry: String,
    pub domain: String,
}

