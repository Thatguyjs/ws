mod client;
pub mod response;
mod status;

use client::Clients;
use response::{Response, ResponseBuilder};
pub use status::Status;
use httparse::{Request, EMPTY_HEADER};
use polling::{Poller, Event, PollMode};
use std::{net::{TcpListener, SocketAddr, TcpStream}, io::{self, Read, Write}, time::Instant, rc::Rc};


pub struct Server {
    listener: TcpListener,
    poller: Poller,
    clients: Clients
}

impl Server {
    pub fn bind(addr: SocketAddr) -> io::Result<Self> {
        Ok(Server {
            listener: TcpListener::bind(addr)?,
            poller: Poller::new()?,
            clients: Clients::new()
        })
    }

    pub fn serve_with_state<R: Into<Response>, T>(&mut self, app: Box<dyn Fn(Rc<T>, Request) -> Option<R>>, state: Rc<T>) -> io::Result<()> {
        self.poller.add_with_mode(&self.listener, Event::readable(0), PollMode::Level)?;

        let mut events = Vec::with_capacity(20);
        let mut prev_time = Instant::now();

        loop {
            events.clear();
            self.poller.wait(&mut events, self.clients.next_timeout())?;

            let now = Instant::now();
            self.clients.sub_time(now.duration_since(prev_time));
            prev_time = now;

            if events.len() == 0 {
                self.clients.remove_timed_out();
                continue;
            }

            for ev in &events {
                if ev.key == 0 {
                    let (stream, _) = self.listener.accept()?;

                    if let Err(e) = self.clients.add(stream, &self.poller) {
                        eprintln!("Error Adding Client: {e}");
                    }
                }
                else if let Some(mut stream) = self.clients.get(ev.key) {
                    let mut buf = [0u8; 2048];
                    let bytes_read = stream.read(&mut buf)?;

                    if bytes_read == 0 {
                        if let Err(e) = self.clients.remove(ev.key, &self.poller) {
                            eprintln!("Error Removing Client: {e}");
                        }
                        continue;
                    }

                    let mut headers = [EMPTY_HEADER; 24];
                    let mut req = Request::new(&mut headers);

                    if let Ok(httparse::Status::Complete(_)) = req.parse(&buf[0..bytes_read]) {
                        if let Some(builder) = app(state.clone(), req) {
                            match builder.into().to_bytes() {
                                Ok(ref res) => stream.write_all(res)?,
                                Err(_) => Self::send_error(&mut stream, Status::InternalServerError)?
                            }
                        }
                    }
                    else {
                        Self::send_error(&mut stream, Status::BadRequest)?;
                    }
                }
            }
        }
    }

    fn send_error(stream: &mut TcpStream, status: Status) -> io::Result<()> {
        let res = ResponseBuilder::new()
            .status(status)
            .into_response();

        stream.write_all(&res.to_bytes()?)
    }
}
