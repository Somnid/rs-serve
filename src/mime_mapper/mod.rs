use std::collections::HashMap;

#[derive(Clone)]
pub struct MimeMapper {
    types : HashMap<&'static str, &'static str>
}

impl MimeMapper {
    pub fn new() -> Self {
        let mut mime_types = HashMap::new();
        mime_types.insert("html", "text/html");
        mime_types.insert("css", "text/css");
        mime_types.insert("txt", "text/plain");
        mime_types.insert("js", "application/javascript");
        mime_types.insert("json", "application/json");
        mime_types.insert("woff", "application/font-woff");
        mime_types.insert("woff2", "application/font-woff2");
        mime_types.insert("svg", "image/svg+xml");
        mime_types.insert("gif", "image/gif");
        mime_types.insert("jpg", "image/jpeg");
        mime_types.insert("jpeg", "image/jpeg");
        mime_types.insert("png", "image/png");
        mime_types.insert("ico", "image/x-icon");
        mime_types.insert("map", "application/json");
        mime_types.insert("mp3", "audio/mpeg");
        mime_types.insert("mp4", "video/mp4");

        MimeMapper {
            types : mime_types
        }
    }

    pub fn map_mime_type(&self, filename: &str) -> Option<String> {
        filename.split('.')
            .last()
            .and_then(|ext| self.types.get(ext))
            .map(|type_str| type_str.to_string())
    }
}
