// A TcpListener with additional shutdown capabilities

use std::{net::{self, SocketAddr}, io, sync::{atomic::{AtomicBool, Ordering}, Arc}};


pub struct TcpListener {
    listener: net::TcpListener,
    running: Arc<AtomicBool>
}


impl TcpListener {
    pub fn bind(addr: SocketAddr) -> io::Result<(Self, TcpShutdown)> {
        let running = Arc::new(AtomicBool::new(true));

        Ok((
            Self {
                listener: net::TcpListener::bind(addr)?,
                running: running.clone()
            },
            TcpShutdown(running, addr)
        ))
    }

    pub fn accept(&self) -> io::Result<(net::TcpStream, SocketAddr)> {
        if self.running.load(Ordering::SeqCst) == false {
            Err(io::Error::new(io::ErrorKind::Other, "Server Closed"))
        }
        else {
            self.listener.accept()
        }
    }

    pub fn incoming<'a>(&'a self) -> Incoming<'a> {
        Incoming(self)
    }
}


pub struct Incoming<'a>(&'a TcpListener);

impl<'a> Iterator for Incoming<'a> {
    type Item = io::Result<net::TcpStream>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.running.load(Ordering::SeqCst) == false {
            None
        }
        else {
            Some(self.0.listener.accept().map(|conn| conn.0))
        }
    }
}


pub struct TcpShutdown(Arc<AtomicBool>, SocketAddr);

impl TcpShutdown {
    pub fn shutdown(&mut self) -> io::Result<()> {
        self.0.store(false, Ordering::SeqCst);

        let stream = net::TcpStream::connect(self.1)?;
        stream.shutdown(net::Shutdown::Both)?;

        Ok(())
    }
}
