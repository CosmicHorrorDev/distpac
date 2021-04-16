use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server_url: String,
}
