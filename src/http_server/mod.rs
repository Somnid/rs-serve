use std::io::{ Read, Write };
use std::net::{ TcpListener, TcpStream };
use std::thread;
use std::str;
use super::server_io;
use super::server_config::ServerConfig;
use super::mime_mapper::MimeMapper;
mod http_responder;
mod http_request;

#[derive(Clone)]
pub struct HttpServer {
	base_path: String,
	mime_mapper: MimeMapper
}

impl HttpServer {
	pub fn new(config: ServerConfig) {
		let http_server = HttpServer {
			base_path: config.base_path.clone(),
			mime_mapper: MimeMapper::new()
		};
		let address = config.get_address();
		let listener = TcpListener::bind(address.as_str()).unwrap();

		println!("started listening on {:?}", address);
		for stream_result in listener.incoming(){
			match stream_result {
				Ok(stream) => {
					let connection = http_server.clone();
					thread::spawn(move ||{
						connection.handle_connection(stream)
					});
				},
				Err(e) => println!("Incoming connection failed with: {:?}", e)
			}
		}
	}

	fn handle_connection(&self, mut stream: TcpStream){
		match self.parse(&stream) {
			Ok(request) => {
				let file_path = format!("{}{}", self.base_path, &request.uri);
				match server_io::read_file_bytes(&file_path) {
					Ok(mut file_content) => respond(&mut stream, &mut file_content, self.mime_mapper.map_mime_type(&file_path)),
					Err(error) => respond(&mut stream, &mut error.into_bytes(), Some("text/plain".to_string()))
				}
			},
			Err(error) => respond(&mut stream, &mut error.into_bytes(), Some("text/plain".to_string()))
		}
	}

	fn parse(&self, mut stream: &TcpStream) -> Result<http_request::HttpRequest, String>{
		let mut request_buffer = [0u8; 4096];
		stream.read(&mut request_buffer)
			.map_err(|e| format!("Failed to read stream into buffer: {:?}", e))
			.and_then(|_| {
				String::from_utf8(request_buffer.to_vec())
					.map_err(|e| format!("Failed to parse as utf8: {:?}", e))
			})
			.and_then(|request_string| http_request::HttpRequest::from_http_string(request_string)
				.map_err(|e| format!("Failed to read request with {:?}", e)))
	}
}

fn respond(stream: &mut TcpStream, mut content: &mut Vec<u8>, content_type: Option<String>){
	let response = http_responder::build_response(&mut content, content_type);
	match stream.write_all(response.as_slice()){
		Err(e) => println!("Failed to write to connection with: {:?}", e),
		_ => ()
	}
}
