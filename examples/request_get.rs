use rust_http_parser::http::HTTPRequest;

fn main() {
    let input = std::fs::read_to_string("examples/request_get.txt").unwrap();
    let request = input.parse::<HTTPRequest>().unwrap();
    println!("====\n{input}\n====\n");
    dbg!(request);
}
