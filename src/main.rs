use hex::decode;
use std::{
    io::{self, BufRead},
    str,
};

const DEBUG_PREFIX: &str = "[debug]";

#[derive(Debug, Clone, PartialEq)]
struct ParseError {
    msg: String,
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from stdin");
        if line.starts_with(DEBUG_PREFIX) {
            println!("{}", pprint(line))
        } else {
            println!("{}", line);
        }
    }
}

fn pprint(line: String) -> String {
    let hex_string = str::replace(&line, "[debug] ", "");
    let result = parse_hex_string(&hex_string).unwrap_or(hex_string.to_string());
    format!("[debug] {}", result)
}

fn parse_hex_string(s: &str) -> Result<String, ParseError> {
    if s.starts_with("0x") {
        let value = str::replace(s, "0x", "");
        let decoded_string = decode(value);
        match decoded_string {
            Ok(buf) => Ok(String::from_utf8(buf).expect("found invalid utf8")),
            Err(_ae) => Err(ParseError {
                msg: "not a hex string".to_string(),
            }),
        }
    } else {
        Err(ParseError {
            msg: "not a hex string".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_hex_string, pprint, ParseError};

    #[test]
    fn pprint_ok() {
        assert_eq!(
            pprint("[debug] 0x6d6f7665206465627567206d6164652065617379".to_string()),
            "[debug] move debug made easy"
        );
        assert_eq!(
            pprint("[debug] aaaaabbbbb".to_string()),
            "[debug] aaaaabbbbb"
        );
    }

    #[test]
    fn parse_ok() {
        assert_eq!(
            parse_hex_string("0x6d6f7665206465627567206d6164652065617379"),
            Ok("move debug made easy".to_string())
        );
        assert_eq!(
            parse_hex_string("aaaaabbbbb"),
            Err(ParseError {
                msg: "not a hex string".to_string()
            })
        );
    }
}
