use std::{fmt, net::{SocketAddr, AddrParseError}, path::PathBuf, time::Duration, io, fs, num::ParseIntError, collections::HashMap};


#[derive(Debug)]
pub enum ErrorKind {
    IOError,
    ParseIntError,
    ParseAddrError,
    BadSection,
    MissingSection,
    UnknownSection,
    UnknownEntry
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String
}

impl Error {
    pub fn new<M: Into<String>>(kind: ErrorKind, message: M) -> Self {
        Self {
            kind,
            message: message.into()
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{:?}: {}", self.kind, self.message))
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self {
            kind: ErrorKind::IOError,
            message: e.to_string()
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Self {
            kind: ErrorKind::ParseIntError,
            message: e.to_string()
        }
    }
}

impl From<AddrParseError> for Error {
    fn from(e: AddrParseError) -> Self {
        Self {
            kind: ErrorKind::ParseAddrError,
            message: e.to_string()
        }
    }
}

impl std::error::Error for Error {}


#[derive(Debug)]
pub struct HttpOptions {
    pub directory: PathBuf,
    pub index_file: String,
    pub client_limit: usize,
    pub keep_alive: Duration,
    pub hosts: Vec<SocketAddr>,
    pub redirects: HashMap<String, String>,
    pub routes: HashMap<PathBuf, PathBuf>,
    pub forward: HashMap<String, SocketAddr>
}

impl HttpOptions {
    pub fn find_config() -> Option<PathBuf> {
        let locs = vec![
            Some(String::from('.')),
            std::env::var("HOME").ok()
        ];

        // Collect paths
        let mut paths = Vec::new();

        for loc in locs {
            if let Some(s) = loc {
                if let Ok(path) = PathBuf::from(s).canonicalize() {
                    paths.push(path.join(".wsconfig"));
                }
            }
        }

        // Test paths to find a config file
        let mut paths = paths.into_iter()
            .filter(|path| fs::File::open(path).is_ok());

        paths.next()
    }

    pub fn parse_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Error> {
        let data = fs::read_to_string(path)?;

        let mut section = "";
        let mut options = Self::default();

        for line in data.lines() {
            let line = line.split_once('#').unwrap_or((line, "")).0.trim();

            if line.is_empty() {
                continue;
            }

            if line.starts_with('[') {
                if !line.ends_with(']') {
                    return Err(Error::new(ErrorKind::BadSection, "Invalid section"));
                }

                section = &line[1..(line.len() - 1)];
            }
            else if !section.is_empty() {
                Self::parse_line(&mut options, &section, line)?;
            }

            else {
                return Err(Error::new(ErrorKind::MissingSection, "Section not specified for item"));
            }
        }

        // Remove the default host address if others are specified
        if options.hosts.len() > 1 {
            options.hosts.remove(0);
        }

        Ok(options)
    }

    fn parse_line(options: &mut HttpOptions, section: &str, line: &str) -> Result<(), Error> {
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
                    "client_limit" => options.client_limit = value.parse()?,
                    "keep_alive" => options.keep_alive = Duration::from_secs(value.parse()?),

                    _ => return Err(Error::new(ErrorKind::UnknownEntry, "Unknown [default] entry"))
                }
            },
            "hosts" => {
                let host = line.trim();
                options.hosts.push(host.parse()?);
            },
            "redirects" => {
                let (key, value) = parse_kvp(line, "->")?;
                options.redirects.insert(key.into(), value.into());
            },
            "routes" => {
                let (key, value) = parse_kvp(line, "->")?;
                options.routes.insert(key.into(), value.into());
            },
            "forward" => {
                let (key, value) = parse_kvp(line, "->")?;
                options.forward.insert(key.into(), value.parse()?);
            },

            _ => return Err(Error::new(ErrorKind::UnknownSection, "Unknown section"))
        }

        Ok(())
    }
}

impl Default for HttpOptions {
    fn default() -> Self {
        Self {
            directory: PathBuf::from("./src"),
            index_file: String::from("index.html"),
            client_limit: 128,
            keep_alive: Duration::from_secs(10),
            hosts: vec!["127.0.0.1:8080".parse().unwrap()],
            redirects: HashMap::new(),
            routes: HashMap::new(),
            forward: HashMap::new()
        }
    }
}
