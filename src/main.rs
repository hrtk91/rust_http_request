mod http;

fn main() {
    let mut req = http::HttpRequest::new();
    req.set_address("http://example.com/", 80).unwrap();
    let resp = req.get();
    println!("{}", resp);
}
