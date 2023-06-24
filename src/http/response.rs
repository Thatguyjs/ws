use std::{net::TcpStream, collections::HashMap, io::{self, Write}};


fn camel_case(slice: &str) -> String {
    let mut want_upper = true;
    let mut result = String::new();

    for ch in slice.chars() {
        let ch = match want_upper {
            true => ch.to_ascii_uppercase(),
            false => ch.to_ascii_lowercase()
        };

        result.push(ch);
        want_upper = " -".contains(ch);
    }

    result
}


#[derive(Debug)]
pub enum Status {
    Ok,
    NotFound
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Ok => "200",
            Status::NotFound => "404"
        }.into()
    }
}


#[derive(Debug)]
pub struct Response<'a> {
    stream: &'a TcpStream,
    status: Status,
    headers: HashMap<String, String>
}

impl<'a> Response<'a> {
    pub fn new(stream: &'a TcpStream, status: Status) -> Self {
        Self {
            stream,
            status,
            headers: HashMap::new()
        }
    }

    pub fn set_header<S: ToString>(&mut self, header: S, value: S) -> &mut Self {
        let header = camel_case(&header.to_string());
        self.headers.insert(header, value.to_string());

        self
    }

    pub fn set_headers<S: ToString + Clone>(&mut self, headers: &[(S, S)]) -> &mut Self {
        for header in headers {
            self.set_header(header.0.clone(), header.1.clone());
        }

        self
    }

    pub fn send(&mut self, body: &[u8]) -> io::Result<()> {
        let statusline = format!("HTTP/1.1 {} {:?}\r\n", self.status.to_string(), self.status);
        self.stream.write(statusline.as_bytes())?;

        self.set_default_headers(body.len());
        let headers = self.stringify_headers();
        self.stream.write(headers.as_bytes())?;

        self.stream.write_all(body)?;
        Ok(())
    }

    fn set_default_headers(&mut self, content_length: usize) {
        if !self.headers.contains_key("Content-Length") {
            self.headers.insert("Content-Length".into(), content_length.to_string());
        }
    }

    fn stringify_headers(&self) -> String {
        let mut result = String::new();

        for header in &self.headers {
            result.push_str(&format!("{}: {}\r\n", header.0, header.1));
        }

        match result.len() {
            0 => result.push_str("\r\n\r\n"),
            _ => result.push_str("\r\n")
        }

        result
    }
}
