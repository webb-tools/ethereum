
#[derive(Debug)]
/// Errors exhibited from `read_hex`.
pub enum ParseHexError {
    InvalidCharacter,
    TooLong,
    TooShort,
    Other
}

/// Parses a given hex string and return a list of bytes if
/// succeeded. The string can optionally start by `0x`, which
/// indicates that it is a hex representation.
pub fn read_hex(s: &str) -> Result<Vec<u8>, ParseHexError> {
    if s.starts_with("0x") {
        return read_hex(&s[2..s.len()]);
    }

    if s.len() & 1 == 1 {
        let mut new_s = "0".to_string();
        new_s.push_str(s);
        return read_hex(&new_s);
    }

    let mut res = Vec::<u8>::new();

    let mut cur = 0;
    let mut len = 0;
    for c in s.chars() {
        len += 1;
        let v_option = c.to_digit(16);
        if v_option.is_none() {
            return Err(ParseHexError::InvalidCharacter);
        }
        let v = v_option.unwrap();
        if len == 1 {
            cur += v * 16;
        } else { // len == 2
            cur += v;
        }
        if len == 2 {
            res.push(cur as u8);
            cur = 0;
            len = 0;
        }
    }

    return Ok(res);
}