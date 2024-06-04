#[allow(dead_code)]
pub mod http {
    use std::{
        collections::HashMap,
        io::{BufRead, BufReader, Read},
        str::FromStr,
    };

    #[derive(Debug, Clone)]
    pub struct HTTPRequest {
        request_line: RequestLine,
        headers: Option<HTTPHeaders>,
        body: Option<String>,
    }

    impl<R: Read> TryFrom<BufReader<R>> for HTTPRequest {
        type Error = String;

        fn try_from(reader: BufReader<R>) -> Result<Self, Self::Error> {
            let mut iterator = reader.lines().map_while(Result::ok).peekable();
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
            let mut iterator = s.split(' ');
            let method: Method = iterator
                .next()
                .ok_or("failed to get HTTP method")?
                .parse()?;
            let request_target = iterator
                .next()
                .ok_or("failed to get request target")?
                .to_string();
            let http_version = iterator
                .next()
                .ok_or("failed to get HTTP version")?
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
        pub fn new(iterator: &mut impl Iterator<Item = String>) -> Result<HTTPHeaders, String> {
            let mut headers = HashMap::new();
            for line in iterator {
                if line.is_empty() {
                    break;
                }
                let mut line = line.split(": ");
                let key = line.next().ok_or("failed to get key")?.to_string();
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

    impl FromStr for Method {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "GET" | "get" => Ok(Method::GET),
                "POST" | "post" => Ok(Method::POST),
                "PUT" | "put" => Ok(Method::PUT),
                "DELETE" | "delete" => Ok(Method::DELETE),
                "HEAD" | "head" => Ok(Method::HEAD),
                "OPTIONS" | "options" => Ok(Method::OPTIONS),
                "CONNECT" | "connect" => Ok(Method::CONNECT),
                "TRACE" | "trace" => Ok(Method::TRACE),
                _ => Err(format!("invalid HTTP method: {s}")),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct HTTPResponse {
        status_line: StatusLine,
        headers: Option<HTTPHeaders>,
        body: Option<String>,
    }

    impl<R: Read> TryFrom<BufReader<R>> for HTTPResponse {
        type Error = String;

        fn try_from(reader: BufReader<R>) -> Result<Self, Self::Error> {
            let mut iterator = reader.lines().map_while(Result::ok).peekable();
            let status_line: StatusLine = iterator
                .next()
                .ok_or("failed to get status line")?
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
            Ok(HTTPResponse {
                status_line,
                headers,
                body,
            })
        }
    }

    #[derive(Debug, Clone)]
    pub struct StatusLine {
        http_version: HTTPVersion,
        status_code: StatusCode,
        status_text: String,
    }

    impl FromStr for StatusLine {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut iterator = s.split(' ');
            let http_version: HTTPVersion = iterator
                .next()
                .ok_or("failed to get HTTP version")?
                .parse()?;
            let status_code: StatusCode = iterator
                .next()
                .ok_or("no status code to be parsed")?
                .parse()?;
            let status_text = iterator
                .next()
                .ok_or("failed to get status text")?
                .to_string();
            Ok(StatusLine {
                http_version,
                status_code,
                status_text,
            })
        }
    }

    #[derive(Debug, Clone)]
    struct HTTPVersion(String);
    impl FromStr for HTTPVersion {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.starts_with("HTTP/") {
                Ok(HTTPVersion(s.to_string()))
            } else {
                Err(format!("invalid HTTP Version: {}", s))
            }
        }
    }

    #[derive(Debug, Clone)]
    struct StatusCode(u16);
    impl FromStr for StatusCode {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            s.parse::<u16>()
                .or(Err(format!("error parsing status code: {}", s)))
                .map(StatusCode)
        }
    }
}
