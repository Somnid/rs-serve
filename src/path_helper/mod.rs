pub fn is_index(path: &str) -> bool {
    path.ends_with("/")
}

pub fn normalize_path(raw_path: &str) -> String {
    let path = raw_path.split('?').nth(0).unwrap();
    let abs_path = if is_index(path) {
        format!("{}{}", path, "index.html")
    } else {
        path.to_string()
    };

    abs_path.replace("//", "/").to_string()
}
