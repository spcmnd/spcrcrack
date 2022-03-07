use clap::Parser;
use std::process;
use std::str::FromStr;

mod lib;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap()]
    pub hash: String,

    #[clap()]
    pub wordlist: String,

    #[clap(short, long)]
    pub algorithm: String,
}

fn main() {
    let Args {
        hash,
        wordlist,
        algorithm,
    } = Parser::parse();

    if let Ok(candidates) = lib::read_lines(wordlist) {
        for c in candidates {
            if let Ok(c) = c {
                if let Ok(a) = lib::Algorithm::from_str(algorithm.as_str()) {
                    match a {
                        lib::Algorithm::Md5 => lib::check_md5(&c, &hash),
                        lib::Algorithm::Sha1 => lib::check_sha1(&c, &hash),
                    }
                } else {
                    eprintln!("Error: Algorithm not recognized or not implemented.");
                    process::exit(1);
                }
            }
        }
    }
}
