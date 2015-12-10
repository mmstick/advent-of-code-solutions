/// 12.230s on A4-5000 (1.5 GHz)
extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

const KEY: &'static str = "iwrupvqb";

/// Continually hashes keys until the first key succeeds the `hash_matched()` function.
fn main() { println!("{}", (1..).skip_while(|x| !hash_matched(x)).take(1).next().unwrap()); }

/// Computes the md5_hash of a string and returns true if it starts with six zeroes.
fn hash_matched(input: &usize) -> bool {
    let mut sh = Md5::new();
    sh.input_str(&format!("{}{}", KEY, *input));
    sh.result_str().starts_with("000000")
}
