use super::status::status_message;

use std::{net::TcpStream, collections::HashMap, io::{self, Write}};


pub struct HttpResponse<'a> {
    stream: &'a TcpStream,

    status: usize,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>
}

impl<'a> HttpResponse<'a> {
    pub fn new(stream: &'a TcpStream) -> Self {
        Self {
            stream,

            status: 0,
            headers: HashMap::new(),
            body: None
        }
    }

    pub fn set_status(&mut self, status: usize) -> &mut Self {
        self.status = status;
        self
    }

    pub fn set_header<T: ToString + ?Sized>(&mut self, header: &T, value: &T) -> &mut Self {
        self.headers.insert(header.to_string(), value.to_string());
        self
    }

    pub fn set_headers<T: ToString + Sized>(&mut self, headers: &[(T, T)]) -> &mut Self {
        for (header, value) in headers {
            self.headers.insert(header.to_string(), value.to_string());
        }

        self
    }

    pub fn set_body(&mut self, body: &[u8]) -> &mut Self {
        self.body = Some(Vec::from(body));
        self
    }

    fn header_string(&self) -> String {
        let mut result = String::new();

        for (header, value) in &self.headers {
            result.push_str(format!("{}: {}\r\n", header, value).as_str());
        }

        result
    }

    pub fn send(mut self) -> io::Result<()> {
        self.stream.write(format!(
            "HTTP/1.1 {} {}\r\n{}\r\n",
            self.status,
            status_message(self.status),
            self.header_string(),
        ).as_bytes())?;

        if self.body.is_some() {
            self.stream.write(&self.body.unwrap())?;
        }

        Ok(())
    }
}

