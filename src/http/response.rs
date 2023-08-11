// Response builder & sender

use super::Status;
use std::{collections::HashMap, io::{self, Write}};


fn title_case(string: &str) -> String {
    let mut result = Vec::with_capacity(string.len());
    let mut next_upper = true;

    for ch in string.chars() {
        if next_upper {
            result.push(ch.to_ascii_uppercase());
            next_upper = false;
        }
        else {
            result.push(ch);
        }

        if ch == '-' {
            next_upper = true;
        }
    }

    result.into_iter().collect()
}


#[derive(Debug)]
pub struct ResponseBuilder {
    version: &'static str,
    status: Status,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>
}

impl Default for ResponseBuilder {
    fn default() -> Self {
        ResponseBuilder {
            version: "HTTP/1.1",
            status: Status::Ok,
            headers: HashMap::new(),
            body: None
        }
    }
}

impl Into<Response> for ResponseBuilder {
    fn into(mut self) -> Response {
        if self.headers.get("Content-Length").is_none() {
            let body_len = match self.body {
                Some(ref b) => b.len(),
                None => 0
            };

            self.headers.insert("Content-Length".into(), body_len.to_string());
        }

        Response {
            version: self.version,
            status: self.status,
            headers: self.headers,
            body: self.body
        }
    }
}

#[allow(dead_code)]
impl ResponseBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn version(mut self, version: &'static str) -> Self {
        self.version = version;
        self
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    pub fn header<K: AsRef<str>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.headers.insert(title_case(key.as_ref()), value.into());
        self
    }

    pub fn headers<K: AsRef<str>, V: Into<String>>(mut self, headers: impl IntoIterator<Item = (K, V)>) -> Self {
        for header in headers {
            self = self.header(header.0, header.1);
        }
        self
    }

    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Response {
        self.body = Some(body.into());
        self.into()
    }

    pub fn into_response(self) -> Response {
        self.into()
    }
}


#[allow(dead_code)] // Rust thinks all Response fields are unused
pub struct Response {
    version: &'static str,
    status: Status,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>
}

impl Response {
    pub fn to_bytes(self) -> io::Result<Vec<u8>> {
        let mut bytes = vec![];
        let status: &str = self.status.into();

        bytes.write(self.version.as_bytes())?;
        bytes.write(b" ")?;
        bytes.write(status.as_bytes())?;
        bytes.write(b"\r\n")?;

        for header in self.headers {
            bytes.write(header.0.as_bytes())?;
            bytes.write(b": ")?;
            bytes.write(header.1.as_bytes())?;
            bytes.write(b"\r\n")?;
        }

        bytes.write(b"\r\n")?;

        if let Some(mut body) = self.body {
            bytes.append(&mut body);
        }

        Ok(bytes)
    }
}
