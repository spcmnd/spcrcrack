extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha1::Sha1;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;

  Ok(io::BufReader::new(file).lines())
}

pub fn check_md5(candidate: &str, target: &str) {
  let mut md5_digest = Md5::new();
  md5_digest.input_str(candidate);

  if target == md5_digest.result_str() {
    println!("FOUND! Password: {}", candidate);
  }
}

pub fn check_sha1(candidate: &str, target: &str) {
  let mut sha1_digest = Sha1::new();
  sha1_digest.input_str(candidate);

  if target == sha1_digest.result_str() {
    println!("FOUND! Password: {}", candidate);
  }
}
