// Directory & File router

use std::{collections::HashMap, path::{Path, PathBuf}};


#[derive(Debug)]
pub struct PathMatch<V> {
    dirs: HashMap<PathBuf, V>,
    files: HashMap<PathBuf, V>
}

impl PathMatch<PathBuf> {
    pub fn add(&mut self, from: PathBuf, to: PathBuf) {
        // Use 'to' instead of 'from', because the directory at 'from' might not exist
        if to.is_dir() {
            self.dirs.insert(from, to);
        }
        else {
            self.files.insert(from, to);
        }
    }

    pub fn get(&self, path: &PathBuf) -> Option<PathBuf> {
        if let Some(to) = self.files.get(path) {
            return Some(to.clone());
        }

        for parent in path.ancestors() {
            if let Some(to) = self.dirs.get(parent) {
                return Some(to.join(path.file_name().unwrap()));
            }
        }

        None
    }
}

impl PathMatch<()> {
    pub fn add(&mut self, path: PathBuf, dir: bool) {
        if dir {
            self.dirs.insert(path, ());
        }
        else {
            self.files.insert(path, ());
        }
    }

    pub fn contains<P: AsRef<Path>>(&self, path: P) -> bool {
        self.files.contains_key(path.as_ref()) || {

            for parent in path.as_ref().ancestors() {
                if self.dirs.contains_key(parent) {
                    return true;
                }
            }

            false
        }
    }
}

impl<V> PathMatch<V> {
    pub fn new() -> Self {
        PathMatch {
            dirs: HashMap::new(),
            files: HashMap::new()
        }
    }

}
