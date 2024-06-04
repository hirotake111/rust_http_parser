use std::io::BufReader;

use rust_http_parser::http::HTTPResponse;

fn main() {
    let file = std::fs::File::open("examples/response_post.txt").unwrap();
    let reader = BufReader::new(file);
    let request: HTTPResponse = reader.try_into().unwrap();
    dbg!(request);
}
