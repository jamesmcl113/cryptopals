use crate::bytes_to_hex;
use crate::encoding::xor_byte;

pub fn encode_rxor(s: &str, key: &str) -> String {
    let key_bytes = key.as_bytes();
    let xored_bytes: Vec<u8> = s
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(i, &b)| xor_byte(key_bytes[i % key_bytes.len()], b))
        .collect();

    bytes_to_hex(&xored_bytes)
}

fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    assert_eq!(a.len(), b.len());

    a.iter()
        .zip(b.iter())
        .map(|(&b1, &b2)| (b1 ^ b2).count_ones())
        .sum()
}

pub fn break_rxor() {
    let file_contents: String = std::fs::read_to_string("file2.txt")
        .unwrap()
        .lines()
        .collect();

    let file_bytes = base64::decode(file_contents).unwrap();

    // first find keysize
    let mut best_edit_distance = f32::INFINITY;
    let mut best_keysize = 2;
    for keysize in 2..30 {
        let mut edit_distance = 0.0;
        let mut num_measurements = 0;
        for i in 0..file_bytes.len() {
            if i + 2 * keysize < file_bytes.len() {
                edit_distance += hamming_distance(
                    &file_bytes[i..i + keysize],
                    &file_bytes[i + keysize..i + 2 * keysize],
                ) as f32;
                num_measurements += 1;
            }
        }

        edit_distance /= keysize as f32;
        edit_distance /= num_measurements as f32;
        if edit_distance < best_edit_distance {
            (best_edit_distance, best_keysize) = (edit_distance, keysize);
        }
    }

    println!("{},{}", best_edit_distance, best_keysize);
    println!("{}", file_bytes.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(
            hamming_distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()),
            37
        );
    }

    #[test]
    fn test_repeating_xor() {
        println!(
            "{}",
            encode_rxor(
                "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
                "ICE",
            )
        );
    }
}
