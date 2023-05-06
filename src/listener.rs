use std::{net::{self, SocketAddr}, io, sync::{Arc, atomic::{AtomicBool, Ordering}}};


pub struct TcpListener {
    listener: net::TcpListener,

    shutdown: Arc<AtomicBool>
}

impl TcpListener {
    pub fn bind(address: SocketAddr) -> io::Result<(Self, TcpShutdown)> {
        let shutdown = Arc::new(AtomicBool::new(false));

        Ok((
            TcpListener {
                listener: net::TcpListener::bind(address)?,

                shutdown: shutdown.clone()
            },
            TcpShutdown(shutdown.clone(), address)
        ))
    }

    pub fn incoming(&self) -> TcpIncoming {
        TcpIncoming(self)
    }
}


pub struct TcpIncoming<'a>(&'a TcpListener);

impl<'a> Iterator for TcpIncoming<'a> {
    type Item = io::Result<net::TcpStream>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.listener.accept() {
            Ok((sk, _)) => {
                if self.0.shutdown.load(Ordering::Relaxed) == true {
                    None
                }
                else {
                    Some(Ok(sk))
                }
            },
            Err(e) => Some(Err(e))
        }
    }
}


pub struct TcpShutdown(Arc<AtomicBool>, SocketAddr);

impl TcpShutdown {
    pub fn shutdown(&self) -> io::Result<()> {
        self.0.store(true, Ordering::Relaxed);

        let stream = net::TcpStream::connect(self.1)?;
        stream.shutdown(net::Shutdown::Both)?;

        Ok(())
    }
}
