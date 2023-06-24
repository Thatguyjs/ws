pub mod client;
pub mod request;
pub mod response;

use client::Client;
use super::listener::*;
use std::{net::SocketAddr, io, path::PathBuf, sync::{Arc, atomic::{Ordering, AtomicUsize}}, thread, time::Duration};


#[derive(Debug)]
pub struct HttpOptions {
    pub hosts: Vec<SocketAddr>,
    pub directory: PathBuf,
    pub index_file: String,
    pub client_limit: usize,
    pub keep_alive: Duration
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
}

impl HttpServer {
    pub fn bind(addr: SocketAddr, options: Option<HttpOptions>) -> io::Result<(Self, TcpShutdown)> {
        let (listener, shutdown) = TcpListener::bind(addr)?;

        Ok((
            Self {
                options: Arc::new(options.unwrap_or(HttpOptions::default())),
                listener,
            },
            shutdown
        ))
    }

    pub fn run(&self) {
        let client_count = Arc::new(AtomicUsize::new(0));

        for stream in self.listener.incoming() {
            if client_count.load(Ordering::SeqCst) >= self.options.client_limit {
                continue;
            }

            if let Ok(stream) = stream {
                client_count.fetch_add(1, Ordering::SeqCst);

                let opts = self.options.clone();
                let cli_count = client_count.clone();

                thread::spawn(move || {
                    let t_stream = stream.try_clone().unwrap();
                    let mut client = Client::new(stream, opts);

                    if let Some(time_open) = client.accept() {
                        thread::spawn(move || {
                            thread::sleep(time_open);
                            let _ = t_stream.shutdown(std::net::Shutdown::Both);
                        });

                        loop {
                            if client.is_closed() {
                                cli_count.fetch_sub(1, Ordering::SeqCst);
                                break;
                            }

                            client.accept();
                        }
                    }
                });
            }
        }
    }
}
