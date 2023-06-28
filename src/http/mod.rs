pub mod client;
pub mod options;
pub mod request;
pub mod response;

use client::Client;
pub use options::HttpOptions;
use super::listener::*;
use std::{net::ToSocketAddrs, io, sync::{Arc, atomic::{Ordering, AtomicUsize}}, thread};


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
    listeners: Vec<Arc<TcpListener>>,
    // TODO: tls: Option<()>,
}

impl HttpServer {
    pub fn bind<A: ToSocketAddrs>(addrs: A, options: HttpOptions) -> io::Result<(Self, HttpShutdown)> {
        let mut listeners = Vec::new();
        let mut shutdown = HttpShutdown(Vec::new());

        for addr in addrs.to_socket_addrs()? {
            let (l, s) = TcpListener::bind(addr)?;
            listeners.push(Arc::new(l));
            shutdown.0.push(s);
        }

        Ok((
            Self {
                options: Arc::new(options),
                listeners
            },
            shutdown
        ))
    }

    pub fn run(&self) {
        let client_count = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::new();

        for listener in &self.listeners {
            let count = client_count.clone();
            let listener = listener.clone();
            let opts = self.options.clone();

            handles.push(thread::spawn(move || {
                for stream in listener.incoming() {
                    if count.load(Ordering::SeqCst) >= opts.client_limit {
                        continue; // Reject connections if the limit has been reached
                    }

                    if let Ok(stream) = stream {
                        count.fetch_add(1, Ordering::SeqCst);

                        let opts = opts.clone();
                        let cli_count = count.clone();

                        thread::spawn(move || {
                            let t_stream = stream.try_clone().unwrap();
                            let mut client = Client::new(stream, opts);

                            if let Some(timeout) = client.accept() {
                                // Close the connection after the keep-alive timeout
                                thread::spawn(move || {
                                    thread::sleep(timeout);
                                    let _ = t_stream.shutdown(std::net::Shutdown::Both);
                                });

                                // Keep accepting requests until the socket is closed
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
            }));
        }

        for handle in handles {
            let _ = handle.join();
        }
    }
}


pub struct HttpShutdown(Vec<TcpShutdown>);

impl HttpShutdown {
    pub fn shutdown(&mut self) -> io::Result<()> {
        for shut in &mut self.0 {
            shut.shutdown()?;
        }

        Ok(())
    }
}
