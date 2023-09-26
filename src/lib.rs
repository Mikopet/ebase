pub fn encode(byte: char) -> String {
    let offset: u8 = (byte as u8) / 16;
    let index: u8 = (byte as u8) - (offset * 16);

    String::from_utf8(vec![
        0xF0,          // b 1111 0000
        0x9F,          // b 1001 1111
        0x8C + offset, // b 1000 1100 + offset
        0x80 + index,  // b 1000 0000 + index
    ])
    .unwrap()
}

pub fn decode(byte: char) -> String {
    let bytes: Vec<u8> = byte.to_string().into_bytes();
    if bytes.len() != 4 {
        panic!("not 4-byte unicode");
    }

    String::from_utf8(vec![(bytes[3] - 0x80) + ((bytes[2] - 0x8C) * 16)]).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn code(str: &str, mode: bool) -> String {
        let mut s: String = String::new();
        for c in str.chars() {
            if mode {
                s = s + &*decode(c)
            } else {
                s = s + &*encode(c)
            }
        }
        s
    }

    #[test]
    fn test_encode() {
        // bytes
        assert_eq!(encode(0 as char), "🌀");
        assert_eq!(encode(0xF0 as char), "🛀");
        // chars
        assert_eq!(encode('a'), "💁");
        assert_eq!(encode('A'), "🐁");
        // strings
        assert_eq!(code("hello world", false), "💈💅💌💌💏🎀📇💏📂💌💄");
        assert_eq!(code("\u{1234} \u{4321}", false), "🏄🎀🎁");
        // emojis
        // TODO
        // assert_eq!(code("🌀", false), "🌀"); // TODO: emoji encoding/decoding is buggy
        // binary
        // TODO
        // file
        // TODO
    }

    #[test]
    fn test_decode() {
        // chars
        assert_eq!(decode('💁'), "a");
        assert_eq!(decode('🐁'), "A");
        // strings
        assert_eq!(code("💈💅💌💌💏🎀📇💏📂💌💄", true), "hello world");
        // TODO
        // assert_eq!(code("🏄🎀🎁", true), "\u{1234} \u{4321}");
        // emojis
        // TODO
        // assert_eq!(code("🌀", true), "🌀"); // TODO: emoji encoding/decoding is buggy
        // binary
        // TODO
        // file
        // TODO
    }
}
