extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod server_io;
mod server_config;
mod http_server;
mod text_parser;
mod mime_mapper;

fn main() {
	let config: server_config::ServerConfig = server_io::read_file_json("config.json".to_string()).unwrap();
	http_server::HttpServer::new(config);
}
