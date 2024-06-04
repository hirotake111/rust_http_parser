use rust_http_parser::http::HTTPRequest;

fn main() {
    let file = std::fs::File::open("examples/request_get.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let request: HTTPRequest = reader.try_into().unwrap();
    dbg!(request);
}
