use std::net::{ TcpStream, SocketAddr };
use std::collections::HashMap;
use std::io::{ Read, Write };

pub mod method {
    pub const GET: &'static str = "GET";
    // pub const  POST: &'static str = "POST";
    // pub const  PUT: &'static str = "PUT";
    // pub const  DELTE: &'static str = "DELETE";
}

pub struct HttpRequest {
    headers: HashMap<String, String>,
    address: Option<SocketAddr>,
    body: String,
    url: Option<String>,
}

impl HttpRequest {
    pub fn new() -> HttpRequest {
        HttpRequest {
            headers: HashMap::new(),
            address: None,
            body: String::from(""),
            url: None,
        }
    }

    pub fn get(&mut self) -> String {
        if self.address == None || self.url == None {
            panic!("get: set_address()を先に実行してください。")
        };

        set_content_length(&mut self.headers, &self.body.len().to_string());

        self.add_header("Accept", "text/html");
        // self.add_header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36");

        let method = format!("{} / HTTP/1.1", method::GET);
        let host = format!(
            "Host:{}",
            get_domain(self.url.as_ref().unwrap()).unwrap());
        let raw_request = format!("{}\r\n{}\r\n{}\r\n", method, host, header_to_string(&self.headers));
        println!("---リクエストヘッダ---\r\n{}----------------------", raw_request);

        let addr = self.address.unwrap();
        let mut stream = match TcpStream::connect(addr) {
            Ok(stream) => stream,
            Err(e) => panic!("{:?}", e),
        };

        if let Err(_) = stream.write(&raw_request.as_bytes()) {
            stream.shutdown(std::net::Shutdown::Both).unwrap();
        }

        let mut buf = [ 0u8; 1024*1000];
        if let Err(_) = stream.read(&mut buf) {
            stream.shutdown(std::net::Shutdown::Both).unwrap();
        }

        return String::from_utf8(buf.to_vec()).unwrap();
    }

    pub fn set_address(&mut self, url: &str, port: u16) -> Result<(), String> {
        use std::net::ToSocketAddrs;

        let domain = match get_domain(url) {
            Ok(domain) => domain,
            Err(err) => return Err(format!("{}", err)),
        };
        let addr = format!("{}:{}", domain, port.to_string());
        let mut socket_addrs = match addr.to_socket_addrs() {
            Ok(addrs) => addrs,
            Err(e) => return Err(format!("set_address ドメイン解決に失敗 e:{:?}", e)),
        };
        
        let socket_addr = socket_addrs.next().unwrap();
        self.address = Some(socket_addr);
        self.url = Some(url.to_string());

        Ok(())
    }

    pub fn add_header(&mut self, key: &str, value: &str) -> () {
        self.headers.insert(key.to_string(), value.to_string());
    }

    // pub fn set_body(&mut self, content: String) -> () {
    //     set_content_length(&mut self.headers, &content);
    //     self.body = content;
    // }
}

fn header_to_string(hash_map: &HashMap<String, String>) -> String {
    String::from(
        hash_map.iter().fold(
            String::from(""),
            |prev, (key, value)| {
                prev + &format!("\"{}\":\"{}\"\r\n", key, value)
            }
        )
    )
}

fn set_content_length(headers: &mut HashMap<String, String>, value: &str) {
    if let Some(_) = headers.get("Content-Length") {
        headers.remove("Content-Length").unwrap();
    };
    headers.insert(String::from("Content-Length"), value.to_string());
}

fn get_domain(url: &str) -> Result<String, String> {
    use regex::Regex;

    let captures = match Regex::new(r"^http://(([\w:%#\$&\?\(\)~\.=\+\-]+)(/*))+$").unwrap().captures(url) {
        Some(captures) => captures,
        None => return Err("URLではありません。".to_string()),
    };
    let domain = captures
        .get(2).unwrap()
        .as_str();
    
    Ok(domain.to_string())
}