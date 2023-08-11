// Manage connected clients

use polling::{Poller, Event};
use std::{net::TcpStream, io, collections::{HashMap, VecDeque}, time::Duration};


#[derive(Debug)]
pub struct Clients {
    clients: HashMap<usize, (TcpStream, Duration)>,
    timeouts: VecDeque<usize>, // Sorted timeouts
    avail: Vec<usize>
}

impl Clients {
    pub fn new() -> Self {
        Clients {
            clients: HashMap::new(),
            timeouts: VecDeque::with_capacity(512),
            avail: (1..=512).rev().collect()
        }
    }

    pub fn add(&mut self, stream: TcpStream, poller: &Poller) -> io::Result<usize> {
        let key = self.avail.pop()
            .ok_or(io::Error::new(io::ErrorKind::Other, "Client Limit Reached"))?;

        if let Err(e) = poller.add_with_mode(&stream, Event::readable(key), polling::PollMode::Level) {
            self.avail.push(key); // Re-add the key
            return Err(e);
        }

        self.clients.insert(key, (stream, Duration::from_secs(5)));
        self.timeouts.push_back(key);
        Ok(key)
    }

    fn remove_timeout(&mut self, key: usize) {
        for (i, t) in self.timeouts.iter().enumerate() {
            if t == &key {
                self.timeouts.remove(i);
                break;
            }
        }
    }

    pub fn remove(&mut self, key: usize, poller: &Poller) -> io::Result<TcpStream> {
        let (stream, _) = self.clients.remove(&key)
            .ok_or(io::Error::new(io::ErrorKind::Other, format!("Client {key} Does Not Exist")))?;

        poller.delete(&stream)?;
        self.avail.push(key);
        self.remove_timeout(key);
        Ok(stream)
    }

    pub fn get(&mut self, key: usize) -> Option<&mut TcpStream> {
        self.clients.get_mut(&key).map(|(stream, _)| stream)
    }

    // Subtract a duration from all clients
    pub fn sub_time(&mut self, time: Duration) {
        for cl in self.clients.values_mut() {
            cl.1 = cl.1.saturating_sub(time);
        }
    }

    // Get the next (smallest) timeout
    pub fn next_timeout(&self) -> Option<Duration> {
        Some(self.clients[self.timeouts.front()?].1)
    }

    // Remove all clients with an expired timeout
    pub fn remove_timed_out(&mut self) {
        let mut rem_keys = vec![];

        self.clients.retain(|key, cl| {
            if cl.1.is_zero() {
                rem_keys.push(*key);
                false
            }
            else {
                true
            }
        });

        for key in rem_keys {
            self.avail.push(key);
            self.remove_timeout(key);
        }
    }
}
