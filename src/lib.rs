#[allow(dead_code)]
pub mod http {
    use std::{collections::HashMap, str::FromStr};

    #[derive(Debug, Clone)]
    pub struct HTTPRequest {
        request_line: RequestLine,
        headers: Option<HTTPHeaders>,
        body: Option<String>,
    }

    impl FromStr for HTTPRequest {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut iterator = s.split("\r\n").peekable();
            let request_line = iterator
                .next()
                .ok_or("failed to get request line".to_string())?
                .parse()?;
            let headers = if iterator.peek().is_some() {
                Some(HTTPHeaders::new(&mut iterator)?)
            } else {
                None
            };
            let body = if iterator.peek().is_some() {
                Some(iterator.collect())
            } else {
                None
            };

            Ok(HTTPRequest {
                request_line,
                headers,
                body,
            })
        }
    }

    #[derive(Debug, Clone)]
    pub struct RequestLine {
        method: Method,
        request_target: String,
        http_version: String,
    }
    impl FromStr for RequestLine {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut iterator = s.split(" ").into_iter();
            let method = iterator
                .next()
                .ok_or("failed to get HTTP method".to_string())
                .and_then(|method| match method {
                    "GET" | "get" => Ok(Method::GET),
                    "POST" | "post" => Ok(Method::POST),
                    "PUT" | "put" => Ok(Method::PUT),
                    "DELETE" | "delete" => Ok(Method::DELETE),
                    "HEAD" | "head" => Ok(Method::HEAD),
                    "OPTIONS" | "options" => Ok(Method::OPTIONS),
                    "CONNECT" | "connect" => Ok(Method::CONNECT),
                    "TRACE" | "trace" => Ok(Method::TRACE),
                    _ => Err(format!("invalid HTTP method: {method}")),
                })?;
            let request_target = iterator
                .next()
                .ok_or("failed to get request target".to_string())?
                .to_string();
            let http_version = iterator
                .next()
                .ok_or("failed to get HTTP version".to_string())?
                .to_string();
            Ok(RequestLine {
                method,
                request_target,
                http_version,
            })
        }
    }

    #[derive(Debug, Clone)]
    struct HTTPHeaders(HashMap<String, String>);
    impl HTTPHeaders {
        pub fn new<'a>(
            iterator: &mut impl Iterator<Item = &'a str>,
        ) -> Result<HTTPHeaders, String> {
            let mut headers = HashMap::new();
            for line in iterator {
                if line == "" {
                    break;
                }
                let mut line = line.split(": ");
                let key = line
                    .next()
                    .ok_or("failed to get key".to_string())?
                    .to_string();
                let value = line
                    .next()
                    .ok_or(format!("failed to get value for key: {key}"))?
                    .to_string();
                headers.insert(key, value);
            }
            Ok(HTTPHeaders(headers))
        }
    }

    #[derive(Debug, Clone)]
    pub enum Method {
        GET,
        POST,
        HEAD,
        OPTIONS,
        DELETE,
        PUT,
        CONNECT,
        TRACE,
    }

    pub struct HTTPResponse {
        status_line: StatusLine,
        headers: HTTPHeaders,
        body: Option<Vec<u8>>,
    }
    pub struct StatusLine {
        http_version: String,
        status_code: u16,
        status_text: String,
    }
}
