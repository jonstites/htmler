use encoding_rs::*;


pub fn encode_to_utf8<'a>(bytes: &'a[u8]) -> String {
    let (result, encoding, errors) = UTF_8.decode(bytes);
    result.into_owned()
}

pub enum Tokenizer {
    Data
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn bytes_to_string() {
	let bytes = b"\xFF\xF0\x9F\x98\x81";
	let expectation = "�😁";
        assert_eq!(&encode_to_utf8(bytes), expectation);
    }
}
