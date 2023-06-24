use super::Method;
use std::{fmt, net::TcpStream, io::{self, Read}, error, collections::HashMap};


#[derive(Debug)]
pub enum ErrorKind {
    IOError,
    InvalidData,
    BadFormat,
    NoData,
    TooLarge,
    Other
}


#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String
}

impl Error {
    pub fn new<M: Into<String>>(kind: ErrorKind, message: M) -> Self {
        Self {
            kind,
            message: message.into()
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{:?}: {}", self.kind, self.message))
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self {
            kind: ErrorKind::IOError,
            message: e.to_string()
        }
    }
}

impl error::Error for Error {}


#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub protocol: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>
}

impl Request {
    pub fn from_stream(mut stream: &TcpStream) -> Result<Self, Error> {
        let mut buf = [0u8; 4096];

        let data = match stream.read(&mut buf)? {
            0 => return Err(Error::new(ErrorKind::NoData, "Request did not contain data")),
            4096 => return Err(Error::new(ErrorKind::TooLarge, "Request contains too much data")),
            c => &buf[0..c]
        };

        let header_end = (|| {
            for (i, wind) in data.windows(4).enumerate() {
                if wind == b"\r\n\r\n" {
                    return Some(i);
                }
            }
            None
        })().ok_or(Error::new(ErrorKind::BadFormat, "Missing body separator"))?;

        let (header, body) = data.split_at(header_end);
        let header = std::str::from_utf8(header)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;
        let body = &body[4..]; // Remove the separator newlines

        let mut req = Self {
            method: Method::Get,
            path: "".into(),
            protocol: "".into(),
            headers: HashMap::new(),
            body: body.to_vec()
        };

        Self::parse_header(&mut req, header)?;
        Ok(req)
    }

    fn parse_header(req: &mut Request, data: &str) -> Result<(), Error> {
        let mut lines = data.lines();

        // Status Line
        let statline = lines.next().ok_or(Error::new(ErrorKind::BadFormat, "Missing Status Line"))?;
        let statline: Vec<&str> = statline.split(' ').collect();

        if statline.len() != 3 {
            return Err(Error::new(ErrorKind::BadFormat, "Invalid Status Line"));
        }

        req.method = Method::from_str(statline[0])
            .ok_or(Error::new(ErrorKind::BadFormat, "Invalid Request Method"))?;
        req.path = statline[1].into();
        req.protocol = statline[2].into();

        // Headers
        for line in lines {
            let (header, value) = line.split_once(':').unwrap_or((line, ""));
            let header = header.trim().to_lowercase();
            let value = value.trim();

            req.headers.insert(header.into(), value.into());
        }

        Ok(())
    }
}
