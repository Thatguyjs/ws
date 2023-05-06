use std::{io::{Read, self}, net::TcpStream, collections::HashMap};


#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch
}

impl HttpMethod {
    pub fn parse(method: &str) -> io::Result<HttpMethod> {
        use HttpMethod::*;
        let method = method.to_uppercase();

        match method.as_str() {
            "GET" => Ok(Get),
            "HEAD" => Ok(Head),
            "POST" => Ok(Post),
            "PUT" => Ok(Put),
            "DELETE" => Ok(Delete),
            "CONNECT" => Ok(Connect),
            "OPTIONS" => Ok(Options),
            "TRACE" => Ok(Trace),
            "PATCH" => Ok(Patch),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid HTTP method"))
        }
    }
}


#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: String,

    pub headers: HashMap<String, String>,
    pub body: String
}

impl HttpRequest {
    pub fn new(stream: &mut TcpStream) -> io::Result<Self> {
        let mut data = [0u8; 4096];
        let mut bytes_read = stream.read(&mut data)?;

        if !data.contains(&b'\n') {
            bytes_read += stream.read(&mut data[bytes_read..])?;
        }

        let data = std::str::from_utf8(&data[0..bytes_read]).unwrap().to_string();

        let (status_line, data) = data.split_once("\r\n").unwrap();
        let status_line: Vec<&str> = status_line.split(' ').collect();

        let (header_data, body) = match data.split_once("\r\n\r\n") {
            Some(data) => data,
            None => (data, "")
        };

        let mut headers = HashMap::new();

        for line in header_data.lines() {
            match line.split_once(':') {
                Some((header, value)) => {
                    headers.insert(header.trim().into(), value.trim().into());
                },
                None => return Err(io::Error::new(io::ErrorKind::Other, "Invalid HTTP header"))
            }
        }

        Ok(Self {
            method: HttpMethod::parse(status_line[0])?,
            path: status_line[1].into(),
            version: status_line[2].into(),

            headers,
            body: body.into()
        })
    }
}
