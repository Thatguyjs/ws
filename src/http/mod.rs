mod status;

pub mod mime;
pub mod request;
pub mod response;

use request::HttpRequest;
use response::HttpResponse;
use crate::listener::{TcpListener, TcpShutdown};

use std::{net::SocketAddr, io};


pub struct HttpServer {
    listener: TcpListener,
}

impl HttpServer {
    pub fn bind(address: SocketAddr) -> io::Result<(Self, HttpShutdown)> {
        let (listener, shutdown) = TcpListener::bind(address)?;

        Ok((
            HttpServer {
                listener,
            },
            HttpShutdown(shutdown)
        ))
    }

    pub fn listen<F: Fn(HttpRequest, HttpResponse)>(&self, f: F) {
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();

            match HttpRequest::new(&mut stream) {
                Ok(req) => f(req, HttpResponse::new(&stream)),
                Err(e) => {
                    eprintln!("Request Error: {}", e);
                }
            }
        }
    }
}


pub struct HttpShutdown(TcpShutdown);

impl HttpShutdown {
    pub fn shutdown(&self) -> io::Result<()> {
        self.0.shutdown()
    }
}
