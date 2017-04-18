pub fn is_index(path: &str) -> bool {
    path.ends_with("/")
}

pub fn normalize_path(path: &str) -> String {
    if is_index(path) {
        format!("{}{}", path, "index.html").to_string()
    } else {
        path.to_string()
    }
}
