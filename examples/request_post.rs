use std::io::BufReader;

use rust_http_parser::http::HTTPRequest;

fn main() {
    let file = std::fs::File::open("examples/request_post.txt").unwrap();
    let reader = BufReader::new(file);
    let request: HTTPRequest = reader.try_into().unwrap();
    dbg!(request);
}
