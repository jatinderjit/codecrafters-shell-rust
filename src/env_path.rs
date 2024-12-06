use std::{
    env,
    path::{Path, PathBuf},
};

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

pub(crate) fn home_dir() -> PathBuf {
    env::var("HOME").unwrap().into()
}

pub(crate) fn expand_home<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    if path.starts_with("~") {
        home_dir().join(path.strip_prefix("~").unwrap())
    } else {
        path.to_owned()
    }
}
