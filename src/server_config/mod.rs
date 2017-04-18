#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    #[serde(rename = "basePath")]
    pub base_path: String,
    pub host: String,
    pub port: u32
}

impl ServerConfig {
    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
