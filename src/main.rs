use clap::Parser;

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
                if algorithm == "md5" {
                    lib::check_md5(&c, &hash);
                } else if algorithm == "sha1" {
                    lib::check_sha1(&c, &hash);
                }
            }
        }
    }
}
