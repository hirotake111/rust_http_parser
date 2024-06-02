# rust_http_parser

HTTP parser in Rust with no dependencies.

### Usage

Both `HTTPRequest` and `HTTPResponse` implement `FromStr` triait and your can `.parse()` to get them.

Example:

```rust
use rust_http_parser::http::HTTPRequest;

fn main() {
    let input = std::fs::read_to_string("examples/request_get.txt").unwrap();
    let request = input.parse::<HTTPRequest>().unwrap();
    dbg!(request);
}
```

The following commands demonstrate what the results look like.

```bash
cargo run --example request_get   # GET  request
cargo run --example request_post  # POST request
cargo run --example response_get  # GET  response
cargo run --example response_post # POST response
```

[MDN HTTP Messages](https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages)
