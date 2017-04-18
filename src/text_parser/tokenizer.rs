pub trait Tokenizer<'a> {
	type TokenIter: Iterator<Item = Token<'a>>;

	fn tokenize(&self, input: &'a str) -> Self::TokenIter;
}

#[derive(Debug,PartialEq)]
pub struct Token<'a> {
	pub term: &'a str,
	pub start_offset: usize,
	pub position: usize
}

impl<'a> Token<'a> {
	pub fn new(term: &'a str, start_offset: usize, position: usize) -> Self {
		Token {
			term: term,
			start_offset: start_offset,
			position: position
		}
	}
}

pub struct CharTokenIter<'a> {
	filter: fn(&(usize, (usize, char))) -> bool,
	input: &'a str,
	byte_offset: usize,
	char_offset: usize,
	position: usize
}

impl<'a> CharTokenIter<'a> {
	pub fn new(filter: fn(&(usize, (usize, char))) -> bool, input: &'a str) -> Self {
		CharTokenIter { filter: filter, input: input, byte_offset: 0, char_offset: 0, position: 0 }
	}
}

impl<'a> Iterator for CharTokenIter<'a> {
	type Item = Token<'a>;

	fn next(& mut self) -> Option<Token<'a>> {
		let mut skipped_bytes = 0;
		let mut skipped_chars = 0;
		for (cidx, (bidx, c)) in self.input[self.byte_offset..]
			.char_indices()
			.enumerate()
			.filter(&self.filter){
				let char_len = c.len_utf8();

				if cidx - skipped_chars == 0 {
					self.byte_offset = self.byte_offset + char_len;
					self.char_offset += 1;
					skipped_bytes = skipped_bytes + char_len;
					skipped_chars += 1;
					continue;
				}

				let slice = &self.input[self.byte_offset..self.byte_offset + bidx - skipped_bytes];
				let token = Token::new(slice, self.char_offset, self.position);
				self.char_offset = self.char_offset + slice.chars().count() + 1;
				self.position += 1;
				self.byte_offset = self.byte_offset + bidx + char_len - skipped_bytes;
				return Some(token);
		}
		if self.byte_offset < self.input.len() {
			let slice = &self.input[self.byte_offset..];
			let token = Token::new(slice, self.char_offset, self.position);
			self.byte_offset = self.input.len();
			Some(token)
		}else{
			None
		}
	}
}

pub struct WhitespaceTokenizer;

impl<'a> Tokenizer<'a> for WhitespaceTokenizer {
	type TokenIter = CharTokenIter<'a>;

	fn tokenize(&self, input: &'a str) -> Self::TokenIter {
		CharTokenIter::new(is_whitespace, input)
	}
}

fn is_whitespace(input: &(usize, (usize, char))) -> bool {
	let (_, (_, c)) = *input;
	c.is_whitespace()
}

#[test]
fn should_split_between_words() {
	let expected = vec![Token::new("hello", 0, 0), Token::new("world", 6, 1)];
	let actual = WhitespaceTokenizer.tokenize("hello world").collect::<Vec<Token>>();
	assert_eq!(expected, actual);
}
