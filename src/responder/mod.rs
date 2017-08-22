use hyper::server;
use hyper::client;
use hyper::uri::RequestUri::AbsolutePath;
use hyper::header::*;
use hyper::status::StatusCode;
use std::io::{Read,Write};
use std::collections::HashMap;

use super::server_config::{ServerConfig,ProxyConfig};
use super::path_helper;
use super::server_io;
use super::traits::is_empty::IsEmpty;
use super::mime_mapper::*;

pub struct Responder {
    base_path: String,
    proxy: ProxyConfig,
    headers: HashMap<String, String>
}

impl Responder {
    pub fn new(config: ServerConfig) -> Self {
        Responder {
            proxy: config.proxy.clone(),
            base_path : config.base_path,
            headers: config.global_headers.clone()
        }
    }
    fn respond_success(&self, mut res: server::Response, content: &[u8]) {
        *res.status_mut() = StatusCode::Ok;
        res.headers_mut()
            .set(ContentLength(content.len() as u64));
        for (key, val) in self.headers.iter() {
            res.headers_mut()
                .set_raw(key.clone(), vec![val.clone().into_bytes()])
        }
        let mut res_body = res.start().unwrap();
        res_body.write_all(content).unwrap();
    }
    fn respond_fail(&self, mut res: server::Response, message: &str) {
        *res.status_mut() = StatusCode::InternalServerError;
        res.headers_mut()
            .set(ContentLength(message.len() as u64));
        let mut res_body = res.start().unwrap();
        res_body.write_all(message.as_bytes()).unwrap();
    }
    fn respond_not_found(&self, mut res: server::Response, message: &str) {
        *res.status_mut() = StatusCode::NotFound;
        res.headers_mut()
            .set(ContentLength(message.len() as u64));
        let mut res_body = res.start().unwrap();
        res_body.write_all(message.as_bytes()).unwrap();
    }
    fn content_result(&self, mut res: server::Response, uri: &str) {
        let file_path = path_helper::normalize_path(&format!("{}{}", self.base_path, uri));
        let content_result = server_io::read_file_bytes(file_path);

        if let Ok(content) = content_result {
            res.headers_mut()
                .set_raw("Content-Type", vec![map_mime_type(uri).as_bytes().to_vec()]);
            self.respond_success(res, &content);
        }  else {
            self.respond_not_found(res, &format!("could not find file {}", &uri));
        }
    }
    fn proxy_result(&self, res: server::Response, uri: &str){
        let client = client::Client::new();
        let final_path = uri.replacen(&self.proxy.endpoint, "", 1);
        println!("Got proxy {} + {} => {}", self.proxy.endpoint, self.proxy.destination, final_path);
        let proxy_address = format!("{}/{}", self.proxy.destination, final_path);
        println!("{}", proxy_address);

        let mut content = Vec::new();
        match client.get(&proxy_address).send() {
            Ok(mut result) => {
                match result.read_to_end(&mut content) {
                    Ok(_) => self.respond_success(res, &content),
                    Err(e) => println!("Proxy failed with {}", e)
                }
            }
            Err(e) => println!("Request failed with {}", e)
        }
    }
}

impl server::Handler for Responder {
    fn handle(&self, req: server::Request, res: server::Response) {
        if let AbsolutePath(uri) = req.uri {
            if !self.proxy.is_empty() && uri.starts_with(&self.proxy.endpoint) {
                self.proxy_result(res, &uri);
            } else {
                self.content_result(res, &uri);
            }
        } else {
            self.respond_fail(res, "not an absolute uri");
        }
    }
}
