// Struct for serving static files

use crate::{path::PathMatch, http::{response::{ResponseBuilder, Response}, Status}};
use percent_encoding::percent_decode_str;
use std::{path::{Path, PathBuf}, io, fs};


fn mime_from_path(path: &Path) -> Option<&str> {
    match path.extension()?.to_str()? {
        "htm" | "html" => Some("text/html"),
        "css" => Some("text/css"),
        "js" => Some("text/javascript"),
        "mjs" => Some("application/javascript"),

        "png" => Some("image/png"),
        "jpg" | "jpeg" => Some("image/jpeg"),
        "ico" => Some("image/x-icon"),

        _ => None
    }
}


pub struct ServeDir {
    path: PathBuf,
    routes: PathMatch<PathBuf>
}

impl ServeDir {
    pub fn new<P: AsRef<Path>>(path: P, routes: PathMatch<PathBuf>) -> Self {
        ServeDir {
            path: path.as_ref().to_owned(),
            routes
        }
    }

    pub fn serve(&self, path: &str) -> Response {
        let path = match percent_decode_str(path.trim_start_matches('/')).decode_utf8() {
            Ok(p) => p,
            Err(_) => return ResponseBuilder::new()
                .status(Status::BadRequest)
                .body("Bad Request")
        };

        let mut file_path = self.path.clone();
        file_path.push(path.as_ref());

        // Auto-request 'index.html' for directory requests
        if file_path.is_dir() {
            file_path.push("index.html");
        }

        // Reroute files
        if let Some(route) = self.routes.get(&file_path) {
            file_path = route;
        }

        match fs::read(&file_path) {
            Ok(data) => {
                let mut res = ResponseBuilder::new()
                    .status(Status::Ok);

                if let Some(mime) = mime_from_path(&file_path) {
                    res = res.header("Content-Type", mime);
                }

                res.body(data)
            },
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                ResponseBuilder::new()
                    .status(Status::NotFound)
                    .body("Not Found")
            },
            Err(e) => {
                eprintln!("Error Serving Path: {e}");

                ResponseBuilder::new()
                    .status(Status::InternalServerError)
                    .body("Internal Server Error")
            }
        }
    }
}
