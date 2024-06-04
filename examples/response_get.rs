use std::io::BufReader;

use rust_http_parser::http::HTTPResponse;

fn main() {
    let file = std::fs::File::open("examples/response_get.txt").unwrap();
    let reader = BufReader::new(file);
    let request = HTTPResponse::try_from(reader).unwrap();
    dbg!(request);
}
