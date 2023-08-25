// Parse server configurations

use crate::path::PathMatch;
use clap::{arg, Arg};
use std::{path::PathBuf, collections::HashMap, net::{SocketAddr, ToSocketAddrs}, io::{ErrorKind, self}};


// Used in .wsconfig files to separate configuration options
enum ConfigSection {
    None,
    Global,
    Redirects,
    Ignore,
    Routes
}

impl TryFrom<&str> for ConfigSection {
    type Error = io::Error;

    fn try_from(section: &str) -> Result<Self, Self::Error> {
        Ok(match section {
            "global" => Self::Global,
            "redirects" => Self::Redirects,
            "ignore" => Self::Ignore,
            "routes" => Self::Routes,
            s => return Err(io::Error::new(ErrorKind::Other, format!("Invalid section name: {s}")))
        })
    }
}


#[derive(Debug)]
pub struct ServerConfig {
    pub address: SocketAddr,
    pub dir: PathBuf,
    pub redirects: HashMap<String, String>,
    pub ignored: PathMatch<()>,
    pub routes: PathMatch<PathBuf>
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            address: "localhost:8080".to_socket_addrs().unwrap().next().unwrap(),
            dir: "./src".into(),
            redirects: HashMap::new(),
            ignored: PathMatch::new(),
            routes: PathMatch::new()
        }
    }
}

impl ServerConfig {
    fn load_cli(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        let cli = clap::Command::new("ws")
            .args([
                arg!(-H --host <IP> "Server host address"),
                arg!(-p --port <PORT> "Server host port"),
                arg!(-d --dir <PATH> "Hosted directory"),
                Arg::new("ignore-file").long("ignore-file").short('i').value_name("URL").help("Ignore requests to a single file, send no response"),
                Arg::new("ignore-dir").long("ignore-dir").short('I').value_name("URL").help("Ignore requests in a directory, send no response"),
                Arg::new("redirect").long("redirect").short('r').value_names(["FROM", "TO"]).help("Redirect URLs"),
                Arg::new("route").long("route").short('R').value_names(["FROM", "TO"]).help("Redirect file paths")
            ])
            .get_matches();

        if let Some(ip) = cli.get_one::<String>("host") {
            self.address = SocketAddr::new(ip.parse()?, self.address.port());
        }
        if let Some(port) = cli.get_one::<String>("port") {
            self.address = SocketAddr::new(self.address.ip(), port.parse::<u16>()?);
        }
        if let Some(dir) = cli.get_one::<String>("dir") {
            self.dir = dir.into();
        }
        if let Some(ignored) = cli.get_many::<String>("ignore-file") {
            for ignore in ignored {
                self.ignored.add(ignore.into(), false);
            }
        }
        if let Some(ignored) = cli.get_many::<String>("ignore-dir") {
            for ignore in ignored {
                self.ignored.add(ignore.into(), true);
            }
        }
        if let Some(redirects) = cli.get_occurrences::<String>("redirect") {
            for mut redir in redirects {
                self.redirects.insert(redir.next().unwrap().into(), redir.next().unwrap().into());
            }
        }
        if let Some(routes) = cli.get_occurrences::<String>("route") {
            for mut route in routes {
                self.routes.add(route.next().unwrap().into(), route.next().unwrap().into());
            }
        }

        Ok(self)
    }

    fn load_file(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        match std::fs::read_to_string(".wsconfig") {
            Ok(data) => {
                let mut section = ConfigSection::None;

                for line in data.lines() {
                    let line = line.split_once('#').unwrap_or((line, "")).0.trim();

                    if line.len() == 0 {
                        continue;
                    }

                    if line.starts_with('[') && line.ends_with(']') {
                        let line = &line[1..(line.len() - 1)];
                        section = ConfigSection::try_from(line)?;
                    }
                    else {
                        match section {
                            ConfigSection::Global => {
                                let (key, val) = line.split_once(':')
                                    .ok_or(io::Error::new(ErrorKind::Other, "Invalid global config line"))?;

                                match key {
                                    "address" => {
                                        let addr = val.trim().trim_matches('"').to_socket_addrs()?;
                                        self.address = addr.clone().next()
                                            .ok_or(io::Error::new(ErrorKind::Other, "Missing address in config"))?;
                                    },
                                    "dir" => self.dir = val.trim().trim_matches('"').into(),
                                    _ => return Err(Box::new(io::Error::new(ErrorKind::Other, "Invalid global config option")))
                                }
                            },
                            ConfigSection::Redirects => {
                                let (from, to) = line.split_once("->")
                                    .ok_or(io::Error::new(ErrorKind::Other, "Invalid redirect config line"))?;

                                self.redirects.insert(
                                    from.trim().trim_matches('"').to_owned(),
                                    to.trim().trim_matches('"').to_owned());
                            },
                            ConfigSection::Ignore => {
                                let ignored: PathBuf = line.trim().trim_matches('"').into();
                                let is_dir = ignored.is_dir();

                                self.ignored.add(ignored, is_dir);
                            },
                            ConfigSection::Routes => {
                                let (from, to) = line.split_once("->")
                                    .ok_or(io::Error::new(ErrorKind::Other, "Invalid route config line"))?;

                                self.routes.add(
                                    from.trim().trim_matches('"').into(),
                                    to.trim().trim_matches('"').into());
                            },

                            ConfigSection::None =>
                                return Err(Box::new(io::Error::new(ErrorKind::Other, "Config line not associated with section")))
                        }
                    }
                }

                Ok(self)
            },
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(self),
            Err(e) => Err(Box::new(e))
        }
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(ServerConfig::default()
            .load_cli()?
            .load_file()?)
    }
}
