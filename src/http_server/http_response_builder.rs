use std::collections::HashMap;
use super::super::server_config::ServerConfig;

pub struct HttpResponseBuilder {
    headers: HashMap<String, String>
}

impl HttpResponseBuilder {
    pub fn new (config: &ServerConfig) -> Self {
        HttpResponseBuilder {
            headers: config.global_headers.clone()
        }
    }

    pub fn build_response(&self, mut content: &mut Vec<u8>, content_type: Option<String>) -> Vec<u8> {
        let mut buffer = get_status();
        let mut header_map = self.headers.clone();

        header_map.insert("Content-Length".to_string(), content.len().to_string());
        content_type.and_then(|content_type_str| header_map.insert("Content-Type".to_string(), content_type_str.to_string()));

        let mut headers = get_headers(header_map);
        buffer.append(&mut headers);
        buffer.append(&mut content);
        buffer
    }
}

pub fn get_status() -> Vec<u8> {
	let status = "HTTP/1.1 200 OK\n";
	status.as_bytes().to_vec()
}

pub fn get_headers(headers: HashMap<String,String>) -> Vec<u8> {
    let mut header_string = String::new();

    for (key, val) in headers.iter() {
        header_string.push_str(&format_header(key, val));
    }
    header_string.push_str("\n");

    header_string.into_bytes()
}

pub fn format_header(key: &str, value: &str) -> String {
    format!("{}: {}\n", key, value)
}
