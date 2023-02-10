fn hex_string_to_bytes(input: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    let mut result: Vec<u8> = vec![];

    for i in 0..(input.len() / 2) {
        let byte = &input[i * 2..(i * 2) + 2];

        println!("{byte:?}");

        result.push(u8::from_str_radix(byte, 16).unwrap());

        println!("{result:?}");
    }
    
    println!("{result:?}");

    Ok(result)
}

pub fn hex_to_base64(input: &str) -> String {
    let base64_alphabet =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

    let bytes = hex_string_to_bytes(input).unwrap();

    let result = bytes
        .iter()
        .map(|byte| {
            let byte = byte.to_owned() as usize;

            // println!("{byte}");
            // println!("{bytes:?}");

            base64_alphabet.chars().nth(byte).unwrap()
        })
        .collect::<String>();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_string_to_bytes() {
        let input = "616263";
        let expected_output: Result<Vec<u8>, std::num::ParseIntError> = Ok(vec![0x61, 0x62, 0x63]);

        assert_eq!(hex_string_to_bytes(input), expected_output);
    }

    #[test]
    fn test_hex_to_base64() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected_output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(hex_to_base64(input), expected_output);
    }
}
