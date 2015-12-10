/// 17.231s on A4-5000 (1.5 GHz)
extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() { println!("{}", get_answer("iwrupvqb")); }

/// Computes the md5_hash of a string.
fn md5_hash(input: &str) -> String {
    let mut sh = Md5::new();
    sh.input_str(input);
    String::from(sh.result_str())
}

/// Returns true if the hashed key starts with six zeroes.
fn check_hash(key: &str) -> bool { !md5_hash(key).starts_with("000000") }

/// Continually hashes keys until the first key succeeds the `check_hash()` function.
fn get_answer(input: &str) -> usize {
    (1..).skip_while(|x| check_hash(&format!("{}{:05}", input, x))).take(1).next().unwrap()
}
