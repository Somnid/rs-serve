use std::collections::HashMap;
use super::traits::is_empty::IsEmpty;

#[derive(Deserialize, Debug, Clone)]
pub struct ProxyConfig {
    pub endpoint: String,
    pub destination: String
}

impl IsEmpty for ProxyConfig {
    fn is_empty(&self) -> bool {
        &self.endpoint == "" && &self.destination == ""
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ServerConfig {
    #[serde(rename = "basePath")]
    pub base_path: String,
    pub host: String,
    pub port: u32,
    #[serde(rename = "globalHeaders")]
    #[serde(default = "HashMap::new")]
    pub global_headers: HashMap<String,String>,
    pub proxy: ProxyConfig
}

impl ServerConfig {
    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proxy_with_no_endpoints_is_empty() {
        let proxy = ProxyConfig {
            endpoint: "".to_string(),
            destination: "".to_string()
        };
        assert_eq!(proxy.is_empty(), true);
    }
}
