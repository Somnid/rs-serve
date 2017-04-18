use super::super::text_parser::tokenizer;
use super::super::text_parser::tokenizer::Tokenizer;

#[derive(Debug)]
pub enum HttpMethod {
	Get,
	Post,
	Put,
	Patch,
	Delete,
	Options,
	Head
}

#[derive(Debug)]
pub struct HttpRequest {
	pub http_version : String,
	pub method : HttpMethod,
	pub uri : String
}

impl HttpRequest {
	pub fn from_http_string<'a>(request_text: String) -> Result<HttpRequest, String> {
		let mut token_iter = tokenizer::WhitespaceTokenizer.tokenize(request_text.as_str());
		let mut http_request = HttpRequest::default();

		match token_iter.next()
			.ok_or("Could not find token for request method".to_string())
			.and_then(|token|get_method(token.term)
							 .ok_or("Could not map request method".to_string())
			) {
				Ok(method) => http_request.method = method,
				Err(error) => return Err(error)
		}

		match token_iter.next()
			.ok_or("Could not find token for uri".to_string())
			.and_then(|token|get_resource_uri(token.term)
							 .ok_or("Could not map uri".to_string())
			) {
				Ok(uri) => http_request.uri = uri,
				Err(error) => return Err(error)
		}

		Ok(http_request)
	}
	pub fn default() -> HttpRequest {
		HttpRequest {
			http_version : "1".to_string(),
			method : HttpMethod::Get,
			uri : "/".to_string()
		}
	}
}

fn get_method(text: &str) -> Option<HttpMethod> {
	match text.to_uppercase().as_str() {
		"GET" => Some(HttpMethod::Get),
		"POST" => Some(HttpMethod::Post),
		"PUT" => Some(HttpMethod::Put),
		"PATCH" => Some(HttpMethod::Patch),
		"DELETE" => Some(HttpMethod::Delete),
		"OPTIONS" => Some(HttpMethod::Options),
		"HEAD" => Some(HttpMethod::Head),
		_ => None
	}
}

fn get_resource_uri(text: &str) -> Option<String> {
	Some(text.to_string())
}
