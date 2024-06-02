use rust_http_parser::http::HTTPResponse;

fn main() {
    let input = std::fs::read_to_string("examples/response_post.txt").unwrap();
    let response = input.parse::<HTTPResponse>().unwrap();
    println!("====\n{input}\n====\n");
    dbg!(response);
}
