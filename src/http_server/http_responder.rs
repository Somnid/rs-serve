use std::collections::HashMap;

pub fn build_response(mut content: &mut Vec<u8>, content_type: Option<String>) -> Vec<u8> {
    let mut buffer = get_status();
    let mut header_map: HashMap<String, String> = HashMap::new();

    header_map.insert("Content-Length".to_string(), content.len().to_string());
    content_type.and_then(|content_type_str| header_map.insert("Content-Type".to_string(), content_type_str.to_string()));

    let mut headers = get_headers(header_map);
    buffer.append(&mut headers);
    buffer.append(&mut content);
    buffer
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
