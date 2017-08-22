extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hyper;

mod traits;
mod path_helper;
mod server_config;
mod server_io;
mod responder;

use hyper::server::Server;
use responder::Responder;

fn main() {
	let config: server_config::ServerConfig = server_io::read_file_json("config.json".to_string()).unwrap();
	let responder = Responder::new(config);

	//println!("using config values: {:?}", debug);

	Server::http("127.0.0.1:1340")
		.unwrap()
		.handle(responder)
		.unwrap();
}
