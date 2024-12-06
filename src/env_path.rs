use std::{env, path::PathBuf};

pub(crate) fn env_paths() -> Vec<PathBuf> {
    env::var("PATH")
        .unwrap_or_default()
        .split(':')
        .map(PathBuf::from)
        .collect()
}

pub(crate) fn find_binary(name: &str) -> Option<PathBuf> {
    for path in env_paths().iter() {
        let path = path.join(name);
        if path.is_file() {
            return Some(path.clone());
        }
    }
    None
}
