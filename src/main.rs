#![allow(dead_code)]
mod encoding;
pub mod repeating_key_xor;

use crate::encoding::{bytes_to_hex, hex_to_bytes, xor_bytes};

fn main() {
    repeating_key_xor::break_rxor();
}

fn crack_xor_file() -> (char, String) {
    let file_contents = std::fs::read_to_string("file.txt").unwrap();

    let mut best_score = 0.0;
    let mut message = String::new();
    let mut cipher = '0';

    for line in file_contents.lines() {
        let (key, possible_message, score) = crack_xor_string(line);
        if score > best_score {
            best_score = score;
            message = possible_message;
            cipher = key;
        }
    }

    (cipher, message)
}

/// Get ratio of alphabetic characters in string
fn rate_string(s: &str) -> f32 {
    s.chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .count() as f32
        / s.len() as f32
}

fn crack_xor_string(s: &str) -> (char, String, f32) {
    let mut char_range = (65..91).collect::<Vec<u8>>();
    char_range.extend(97..123);

    let mut best_score: f32 = 0.0;
    let (mut key, mut message): (u8, String) = (0, String::new());

    for c in 0u8..=255 {
        let cipher_text_bytes = hex_to_bytes(s).unwrap();
        let repeated_byte = vec![c; cipher_text_bytes.len()];
        let possible_message = String::from_utf8(xor_bytes(&cipher_text_bytes, &repeated_byte))
            .unwrap_or("0".to_string());
        let rating = rate_string(&possible_message);

        if rating > best_score {
            best_score = rating;
            key = c;
            message = possible_message;
        }
    }

    (key as char, message, best_score)
}

#[cfg(test)]
mod tests {
    use super::encoding::encode_base64;
    use super::*;

    #[test]
    fn test_b64_encode() {
        let b64_string = encode_base64(&hex_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap());
        assert_eq!(
            b64_string,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }

    #[test]
    fn test_hex_encode() {
        assert_eq!(
            xor_bytes(
                &hex_to_bytes("1c0111001f010100061a024b53535009181c").unwrap(),
                &hex_to_bytes("686974207468652062756c6c277320657965").unwrap(),
            ),
            hex_to_bytes("746865206b696420646f6e277420706c6179").unwrap()
        );
    }
}
