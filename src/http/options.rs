use std::{net::SocketAddr, path::PathBuf, time::Duration, io, fs};


#[derive(Debug)]
pub struct HttpOptions {
    pub hosts: Vec<SocketAddr>,
    pub directory: PathBuf,
    pub index_file: String,
    pub client_limit: usize,
    pub keep_alive: Duration
}

impl HttpOptions {
    pub fn parse_file<P: AsRef<std::path::Path>>(path: P) -> io::Result<Self> {
        let data = fs::read_to_string(path)?;

        let mut section = "";
        let mut options = Self::default();

        for line in data.lines() {
            let line = line.split_once('#').unwrap_or((line.trim(), "")).0;

            if line.is_empty() {
                continue;
            }

            if line.starts_with('[') {
                if !line.ends_with(']') {
                    return Err(io::Error::new(io::ErrorKind::Other, "Invalid section"));
                }

                section = &line[1..(line.len() - 1)];
            }
            else if !section.is_empty() {
                Self::parse_line(&mut options, &section, line)?;
            }

            else {
                return Err(io::Error::new(io::ErrorKind::Other, "Section not specified for item"));
            }
        }

        Ok(options)
    }

    fn parse_line(options: &mut HttpOptions, section: &str, line: &str) -> io::Result<()> {
        // Parse and format a single key-value pair
        fn parse_kvp<'a>(line: &'a str, delimiter: &str) -> io::Result<(&'a str, &'a str)> {
            line.split_once(delimiter)
                .ok_or(io::Error::new(io::ErrorKind::Other, "Invalid settings entry"))
                .map(|pair| (pair.0.trim(), pair.1.trim()))
        }

        match section {
            "defaults" => {
                let (key, value) = parse_kvp(line, ":")?;

                match key {
                    "directory" => options.directory = value.trim_matches('"').into(),
                    "index_file" => options.index_file = value.trim_matches('"').into(),
                    "client_limit" => options.client_limit = value.parse().unwrap(),
                    "keep_alive" => options.keep_alive = Duration::from_secs(value.parse().unwrap()),

                    _ => return Err(io::Error::new(io::ErrorKind::Other, "Invalid default option"))
                }
            },
            "hosts" => {
                let host = line.trim();
                // println!("Add host: {host}");
            },
            "redirects" => {
                let (key, value) = parse_kvp(line, "->")?;
                // println!("Redirect: {key} to {value}");
            },
            "routes" => {
                let (key, value) = parse_kvp(line, "->")?;
                // println!("Route: {key} to {value}");
            },
            "forward" => {
                let (key, value) = parse_kvp(line, "->")?;
                // println!("Forward: {key} to {value}");
            },

            _ => return Err(io::Error::new(io::ErrorKind::Other, "Unknown section"))
        }

        Ok(())
    }
}

impl Default for HttpOptions {
    fn default() -> Self {
        Self {
            hosts: Vec::new(),
            directory: PathBuf::from("./src"),
            index_file: String::from("index.html"),
            client_limit: 128,
            keep_alive: Duration::from_secs(10)
        }
    }
}
