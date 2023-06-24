pub mod client;
pub mod request;
pub mod response;

use client::Client;
use super::{listener::*, threadpool::*};
use std::{net::SocketAddr, io, path::PathBuf, sync::Arc, thread};


#[derive(Debug)]
pub struct HttpOptions {
    pub hosts: Vec<SocketAddr>,
    pub directory: PathBuf,
    pub index_file: String,
    pub client_limit: usize
}

impl Default for HttpOptions {
    fn default() -> Self {
        Self {
            hosts: Vec::new(),
            directory: PathBuf::from("../2048/"),
            index_file: String::from("index.html"),
            client_limit: 128
        }
    }
}


#[derive(Debug)]
pub enum Method {
    Get,
    Post
}

impl Method {
    pub fn from_str(value: &str) -> Option<Self> {
        use Method::*;

        match value.to_uppercase().as_str() {
            "GET" => Some(Get),
            "POST" => Some(Post),
            _ => None
        }
    }
}


pub struct HttpServer {
    options: Arc<HttpOptions>,
    listener: TcpListener,
    // TODO: tls: Option<()>,
    clients: Vec<Client>,
    client_rc: Receiver<()>,
    pool: ThreadPool
}

impl HttpServer {
    pub fn bind(addr: SocketAddr, options: Option<HttpOptions>) -> io::Result<(Self, TcpShutdown)> {
        let (listener, shutdown) = TcpListener::bind(addr)?;

        Ok((
            Self {
                options: Arc::new(options.unwrap_or(HttpOptions::default())),
                listener,
                clients: Vec::new(),
                pool: ThreadPool::new(4)
            },
            shutdown
        ))
    }

    pub fn run(&mut self) -> io::Result<()> {
        thread::spawn(move || {

        });

        for stream in self.listener.incoming() {
            if let Ok(stream) = stream {
                if self.clients.len() >= self.options.client_limit {
                    let _ = stream.shutdown(std::net::Shutdown::Both);
                }

                let id = self.clients.len();
                if let Ok(client) = Client::new(id, stream, self.options.clone()) {
                    self.clients.push(client);
                }
            }
        }

        Ok(())
    }
}
