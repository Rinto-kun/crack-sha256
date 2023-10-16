use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
mod algos;
use itertools::Itertools;

pub fn brute_force(
    hashes: Vec<&[u8]>,
    n: usize,
    possible_chars: String,
) -> HashMap<String, String> {
    let mut hashes: Vec<Vec<u8>> = hashes
        .iter()
        .map(hex::decode)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let mut result = HashMap::new();
    let possible_chars = possible_chars.as_bytes();

    for i in 1..=n {
        for p in algos::permutations(possible_chars, i) {
            let hash = Sha256::digest(&p);

            for (idx, h) in hashes.iter().enumerate() {
                if h[..] == hash[..] {
                    let string = String::from_utf8(p).unwrap();
                    // Remove value from hashes list.
                    let el = hashes.swap_remove(idx);

                    // Insert value
                    result.insert(string, hex::encode(el));

                    if hashes.is_empty() {
                        return result;
                    }
                    break;
                }
            }
        }
    }
    result
}

pub fn brute_force_itertools(
    hashes: Vec<&[u8]>,
    n: usize,
    possible_chars: String,
) -> HashMap<String, String> {
    let mut hashes: Vec<Vec<u8>> = hashes
        .iter()
        .map(hex::decode)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let mut result = HashMap::new();

    for i in 1..=n {
        for p in std::iter::repeat(possible_chars.bytes())
            .take(i)
            .multi_cartesian_product()
        {
            let hash = Sha256::digest(&p);

            // hashes.co
            for (idx, h) in hashes.iter().enumerate() {
                if h[..] == hash[..] {
                    let string = String::from_utf8(p).unwrap();

                    // Remove value from hashes list.
                    let el = hashes.swap_remove(idx);

                    // Insert value
                    result.insert(string, hex::encode(el));

                    if hashes.is_empty() {
                        return result;
                    }
                    break;
                }
            }
        }
    }

    result
}

pub fn dictionary_attack(
    dictionary_path: &PathBuf,
    hashes: Vec<&[u8]>,
) -> HashMap<String, String> {
    let f = File::open(dictionary_path).unwrap();
    let f = BufReader::new(f);

    let mut hashes = hashes
        .iter()
        .map(hex::decode)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let mut ret: HashMap<String, String> = HashMap::new();
    for line in f.lines() {
        let line: String = line.unwrap();
        // let line = line.trim();
        let hash = Sha256::digest(&line);

        for (idx, h) in hashes.iter().enumerate() {
            if h[..] == hash[..] {
                // Remove value from hashes list.
                let el = hashes.swap_remove(idx);

                // Insert value
                ret.insert(line.to_string(), hex::encode(el));

                if hashes.is_empty() {
                    return ret;
                }
                break;
            }
        }
    }
    ret
}
