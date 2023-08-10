use std::num::ParseIntError;

const BASE_64_TABLE: [char; 65] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/', '=',
];

pub fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b.iter()).map(|(&x, &y)| x ^ y).collect()
}

pub fn xor_byte(a: u8, b: u8) -> u8 {
    a ^ b
}

pub fn encode_base64(bytes: &[u8]) -> String {
    bytes
        .chunks(3)
        .map(|b| {
            let x = (b[0] as u32) << 16
                | (*b.get(1).unwrap_or(&0) as u32) << 8
                | *b.get(2).unwrap_or(&0) as u32;

            let char_a = (x >> 18) as u8 & 0x3F;
            let char_b = (x >> 12) as u8 & 0x3F;
            let char_c = if b.len() > 1 {
                (x >> 6) as u8 & 0x3F
            } else {
                64
            };
            let char_d = if b.len() == 3 { (x & 0x3F) as u8 } else { 64 };

            [char_a, char_b, char_c, char_d]
        })
        .flatten()
        .map(|num| BASE_64_TABLE[num as usize])
        .collect()
}

pub fn hex_to_bytes(string: &str) -> Result<Vec<u8>, ParseIntError> {
    assert_eq!(
        string.len() % 2,
        0,
        "Hex string must contain an even number of chars."
    );

    (0..string.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&string[i..i + 2], 16))
        .collect()
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|&b| format!("{:02x?}", b))
        .fold(String::new(), |acc, s| acc + s.as_str())
}
