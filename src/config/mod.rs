// Parse server configurations

mod error;
mod file;

use crate::path::PathMatch;
use clap::{arg, Arg, crate_authors, crate_version};
use std::{collections::HashMap, net::{SocketAddr, ToSocketAddrs}, path::PathBuf};

use self::file::ConfigFile;


macro_rules! set_if_default {
    ($prop:expr, $value:expr, $default:expr) => {
        if $prop == $default {
            $prop = $value;
        }
    };
}


#[derive(Debug)]
pub struct ServerConfig {
    pub address: SocketAddr,
    pub dir: PathBuf,
    pub redirects: HashMap<String, String>,
    pub ignored: PathMatch<()>,
    pub routes: PathMatch<PathBuf>,
    no_config: bool
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            address: "localhost:8080".to_socket_addrs().unwrap().next().unwrap(),
            dir: "./src".into(),
            redirects: HashMap::new(),
            ignored: PathMatch::new(),
            routes: PathMatch::new(),
            no_config: false
        }
    }
}

impl ServerConfig {
    fn load_cli(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        let cli = clap::Command::new("ws")
            .version(crate_version!())
            .author(crate_authors!())
            .args([
                arg!(-a --address <ADDRESS> "Server host address"),
                arg!(-d --dir <PATH> "Hosted directory"),
                arg!(-n --noconfig "Don't attempt to load a server config from a file"),
                Arg::new("ignore-file").long("ignore-file").short('i').value_name("URL").help("Ignore requests to a single file, send no response"),
                Arg::new("ignore-dir").long("ignore-dir").short('I').value_name("URL").help("Ignore requests in a directory, send no response"),
                Arg::new("redirect").long("redirect").short('r').value_names(["FROM", "TO"]).help("Redirect URLs"),
                Arg::new("route").long("route").short('R').value_names(["FROM", "TO"]).help("Redirect file paths")
            ])
            .get_matches();

        if let Some(addr) = cli.get_one::<String>("address") {
            self.address = addr.parse()?;
        }
        if let Some(dir) = cli.get_one::<String>("dir") {
            self.dir = dir.into();
        }
        if cli.get_flag("noconfig") {
            self.no_config = true;
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

    fn load_file(mut self) -> error::Result<Self> {
        let cfg = match ConfigFile::read(".wsconfig") {
            Ok(c) => c,
            Err(e) if e.kind == error::ErrorKind::IOError => return Ok(self),
            Err(e) => return Err(e)
        };
        let default = ServerConfig::default();

        for section in cfg.sections() {
            match section.name.as_str() {
                "global" => {
                    if let Some(addr) = section.keys.get("address") {
                        set_if_default!(self.address, addr.to_socket_addrs()?.next().unwrap(), default.address);
                    }
                    if let Some(dir) = section.keys.get("dir") {
                        set_if_default!(self.dir, dir.into(), default.dir);
                    }
                },
                "redirects" => {
                    for (from, to) in &section.keys {
                        self.redirects.insert(from.clone(), to.clone());
                    }
                },
                "routes" => {
                    for (from, to) in &section.keys {
                        self.routes.add(from.into(), to.into());
                    }
                },
                "ignore" => {
                    for (path, _) in &section.keys {
                        // TODO: Check if 'path' is a file or directory
                        self.ignored.add(path.into(), false);
                    }
                },
                "log" => {
                    // TODO: Implement log levels / specific events
                },
                _ => return Err(error::Error::new(error::ErrorKind::InvalidSection, "Unknown section"))
            }
        }

        Ok(self)
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let cfg = ServerConfig::default().load_cli()?;

        match cfg.no_config {
            true => Ok(cfg),
            false => Ok(cfg.load_file()?)
        }
    }
}
