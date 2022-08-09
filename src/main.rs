use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use sha256::{digest};

fn usage() {
    println!("Usage:");
    println!("hash_cracker <relative or absolute path to wordlist file> <hash string that is searched for>");
}

fn main() {

    println!("------------\nHash Cracker\n------------");

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        usage();
        return;
    }

    let hash_val = &args[2];

    //Read entry in wordlist and hash this entry with the chosen hashing algorithm
    //compare result with given hash value

    println!("Searching for password...");

    if let Ok(lines) = read_lines(&args[1]) {

        for line in lines {
            if let Ok(pw_entry) = line {
                //TODO: Calc hash until you find correct one or you reached the final entry in the given wordlist
                let digested_val = digest(&pw_entry);
                
                match digested_val.cmp(hash_val) {

                    Ordering::Equal => {
                        println!("Password is '{}'", pw_entry);
                    },
                    _ => ()
                }

            }
        }
    }

}

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
