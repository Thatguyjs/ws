// Parse server configurations

use clap::{arg, Arg};
use std::{path::PathBuf, collections::HashMap, net::{SocketAddr, ToSocketAddrs}};


#[derive(Debug)]
pub struct ServerConfig {
    pub address: SocketAddr,
    pub dir: PathBuf,
    pub redirects: HashMap<String, String>,
    pub routes: Vec<(String, String)>,
    // TODO: Request forwarding
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            address: "localhost:8080".to_socket_addrs().unwrap().next().unwrap(),
            dir: "./src".into(),
            redirects: HashMap::new(),
            routes: Vec::new()
        }
    }
}

impl ServerConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let cli = clap::Command::new("ws")
            .args([
                arg!(-H --host <IP> "Server host address"),
                arg!(-p --port <PORT> "Server host port"),
                arg!(-d --dir <PATH> "Hosted directory"),
                Arg::new("redirect").long("redirect").short('r').value_names(["FROM", "TO"]).help("Redirect URLs"),
                Arg::new("route").long("route").short('R').value_names(["FROM", "TO"]).help("Redirect file paths")
            ])
            .get_matches();

        let mut cfg = ServerConfig::default();

        if let Some(ip) = cli.get_one::<&str>("host") {
            cfg.address = SocketAddr::new(ip.parse().unwrap(), cfg.address.port());
        }
        if let Some(port) = cli.get_one::<u16>("port") {
            cfg.address = SocketAddr::new(cfg.address.ip(), *port);
        }
        if let Some(dir) = cli.get_one::<String>("dir") {
            cfg.dir = dir.into();
        }
        if let Some(redirects) = cli.get_occurrences::<String>("redirect") {
            for mut redir in redirects {
                cfg.redirects.insert(redir.next().unwrap().into(), redir.next().unwrap().into());
            }
        }
        if let Some(routes) = cli.get_occurrences::<String>("route") {
            for mut route in routes {
                cfg.routes.push((route.next().unwrap().into(), route.next().unwrap().into()));
            }
        }

        Ok(cfg)
    }
}
