extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha1::Sha1;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Algorithm {
  Md5,
  Sha1,
}

impl FromStr for Algorithm {
  type Err = ();

  fn from_str(input: &str) -> Result<Algorithm, Self::Err> {
    match input {
      "md5" => Ok(Algorithm::Md5),
      "sha1" => Ok(Algorithm::Sha1),
      _ => Err(()),
    }
  }
}

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
    display_result(candidate, target);
  }
}

pub fn check_sha1(candidate: &str, target: &str) {
  let mut sha1_digest = Sha1::new();
  sha1_digest.input_str(candidate);

  if target == sha1_digest.result_str() {
    display_result(candidate, target);
  }
}

fn display_result(password: &str, hash: &str) {
  println!("FOUND! Hash: {} - Password: {}", hash, password);
  process::exit(1);
}
