pub fn map_mime_type(path: &str) -> &str {
    match(path.split('.').last()){
        Some(ext) => match(ext){
                "html" => "text/html",
                "css" => "text/css",
                "js" => "application/javascript",
                "json" => "application/json",
                "txt" => "text/plain",
                "woff" => "application/font-woff",
                "woff2" => "font/woff2",
                "svg" => "image/svg+xml",
                "gif" => "image/gif",
                "jpg" => "image/jpeg",
                "jpeg" => "image/jpeg",
                "png" => "image/png",
                "ico" => "image/x-icon",
                "map" => "application/json",
                "mp3" => "audio/mpeg",
                "mp4" => "video/mp4",
                _ => "application/octet-stream"
            },
        None => "application/octet-stream"
    }
}
