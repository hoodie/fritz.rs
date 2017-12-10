extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() {
    let mut digest = Md5::new();
    digest.input_str("signme");
    println!("{}", digest.result_str());
}
