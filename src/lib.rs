use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};
mod algos;

// pub fn brute_force(hashes: HashSet<&str>, n: usize) -> HashMap<String, String> {
//     let possible_chars = "abcdefghijklmnopqrstuvwxyz0123456789";

//     let mut hashes = hashes;

//     let mut result = HashMap::new();

//     for i in 1..=n {
//         for p in algos::permutations(&possible_chars.as_bytes(), i) {

//             let hash = hex::encode(Sha256::new().chain_update(&p).finalize());
//             if hashes.remove(&hash[..]){
//                 result.insert(String::from_utf8(p).unwrap(), hash);
//                 if hashes.is_empty() {return result;}
//             }
//         }
//     }
//     result
// }

pub fn brute_force(mut hashes: Vec<&[u8]>, n: usize) -> HashMap<String, [u8; 32]> {
    let possible_chars = Vec::from_iter("abcdefghijklmnopqrstuvwxyz0123456789".chars());
    let mut result = HashMap::new();

    for i in 1..=n {
        for p in algos::permutations(&possible_chars, i) {
            let string = String::from_iter(p);
            let hash = Sha256::digest(string.as_bytes());

            for (idx, h) in hashes.iter().enumerate() {
                if h[..] == hash[..] {
                    // Insert value
                    result.insert(string, hash.into());

                    // Remove value from hashes list.
                    hashes.swap_remove(idx);

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


// pub fn brute_force(mut hashes: HashSet<&[u8; 64]>, n: usize) -> HashMap<String, String> {
//     let possible_chars = "abcdefghijklmnopqrstuvwxyz0123456789";

//     let mut result = HashMap::new();

//     let mut permutation_buffer = Vec::with_capacity(n);
//     let mut hex_buffer = [0; 64];

//     for i in 1..=n {
//         let mut permutations = algos::permutations(possible_chars.as_bytes(), i);
//         while let Some(permutation) = permutations.next() {
//             permutation_buffer.clear();
//             permutation_buffer.extend(permutation);

//             hex::encode_to_slice(Sha256::digest(&permutation_buffer), &mut hex_buffer).unwrap();

//             if hashes.remove(&hex_buffer) {
//                 let permutation = String::from_utf8(permutation_buffer.clone()).unwrap();
//                 let hash = String::from_utf8(hex_buffer.to_vec()).unwrap();
//                 result.insert(permutation, hash);

//                 if hashes.is_empty() {
//                     return result;
//                 }
//             }
//         }
//     }

//     result
// }

pub fn dict_attack(
    dictionary_address: &str,
    hashes: Vec<&str>,
) -> HashMap<String, String>{
    let f = File::open(dictionary_address).unwrap();
    let f = BufReader::new(f);
    let mut hashes = hashes.clone();

    let mut ret: HashMap<String, String> = HashMap::new();
    for line in f.lines() {
        let line = line.unwrap().trim_end().to_string();
        let hash = hex::encode(Sha256::new().chain_update(line.clone()).finalize());

        for (idx, &h) in hashes.iter().enumerate() {
            if h == &hash {
                // Insert value
                ret.insert(line, hash);

                // Remove value from hashes list.
                hashes.swap_remove(idx);

                if hashes.is_empty() {
                    return ret;
                }
                break;
            }
        }
    }
    ret
}
