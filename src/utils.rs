use std::env;

pub fn get_path_src(path: String) -> String {
    get_path_dir(format!("src/{}", path))
}

pub fn get_path_assets(path: String) -> String {
    get_path_dir(format!("assets/{}", path))
}

pub fn get_path_dir(path: String) -> String {
    let mut new_path = env!("CARGO_MANIFEST_DIR").to_string();
    new_path.push_str(&format!("/{}", path));
    return new_path;
}
