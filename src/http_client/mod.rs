use std::net::TcpStream;
use std::io::Read;

pub struct HttpClient {}

impl HttpClient {
    pub fn request(&str address) -> String {
        let stream = TcpStream::connect(address).unwrap();
        let string = stream.read_to_string();
        println!("{}", string);
    }
}
