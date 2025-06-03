use regev::*;

fn main() {
    let (s, pk) = keygen::keygen();
    for value in 0..4 {
        let ct = encrypt::encrypt(&pk, value);
        let recovered = decrypt::decrypt(&s, &ct);
        println!("Original: {} | Decrypted: {}", value, recovered);
    }
}

#[cfg(test)]
mod tests {
    use crate::{decrypt, encrypt, keygen};

    #[test]
    fn encrypted_and_decrypted_values_are_the_same() {
        let (s, pk) = keygen::keygen();
        for value in 0..4 {
            let ct = encrypt::encrypt(&pk, value);
            let recovered = decrypt::decrypt(&s, &ct);
            assert_eq!(value, recovered);
        }
    }
}

