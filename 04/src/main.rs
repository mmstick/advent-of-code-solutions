/// 12.230s on A4-5000 (1.5 GHz)
extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() { println!("{}", get_answer("iwrupvqb")); }

/// Computes the md5_hash of a string and returns true if it starts with six zeroes.
fn hash_is_match(input: &str) -> bool {
    let mut sh = Md5::new();
    sh.input_str(input);
    sh.result_str().starts_with("000000")
}

/// Continually hashes keys until the first key succeeds the `check_hash()` function.
fn get_answer(input: &str) -> usize {
    (1..).skip_while(|x| !hash_is_match(&format!("{}{:05}", input, x))).take(1).next().unwrap()
}
