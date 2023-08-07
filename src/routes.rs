// Directory & File router

use std::{collections::HashMap, path::{Path, PathBuf}};


#[derive(Debug)]
pub struct Routes {
    dirs: HashMap<PathBuf, PathBuf>,
    files: HashMap<PathBuf, PathBuf>
}

impl Routes {
    pub fn new() -> Self {
        Routes {
            dirs: HashMap::new(),
            files: HashMap::new()
        }
    }

    pub fn add(&mut self, from: PathBuf, to: PathBuf) {
        // Use 'to' instead of 'from', because the directory at 'from' might not exist
        if to.is_dir() {
            self.dirs.insert(from, to);
        }
        else {
            self.files.insert(from, to);
        }
    }

    pub fn get(&self, path: &Path) -> Option<PathBuf> {
        if let Some(to) = self.files.get(path) {
            return Some(to.clone());
        }

        if let Some(dir) = path.parent() {
            if let Some(to) = self.dirs.get(dir) {
                return Some(to.join(path.file_name().unwrap()));
            }
        }

        None
    }
}
