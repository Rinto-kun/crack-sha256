use std::{env, time::Instant, vec};

use crack_sha256::{brute_force, brute_force_itertools, dictionary_attack};

const POSSIBLE_CHARS: &'static str = "abcdefghijklmnopqrstuvwxyz0123456789";

fn task1() {
    let hashes: Vec<&[u8]> = vec![
        b"594e519ae499312b29433b7dd8a97ff068defcba9755b6d5d00e84c524d67b06",
        b"ade5880f369fd9765fb6cffdf67b5d4dfb2cf650a49c848a0ce7be1c10e80b23",
        b"83cf8b609de60036a8277bd0e96135751bbc07eb234256d4b65b893360651bf2",
        b"0d335a3bea76dac4e3926d91c52d5bdd716bac2b16db8caf3fb6b7a58cbd92a7",
    ];

    let now = Instant::now();
    let result = brute_force(hashes.clone(), 4, POSSIBLE_CHARS.to_string());
    println!("BruteForce_Custom took: {:?}", now.elapsed());
    println!("{result:?}\n");

    let now = Instant::now();
    let result = brute_force_itertools(hashes.clone(), 4, POSSIBLE_CHARS.to_string());
    println!("BruteForce_Itertools took: {:?}", now.elapsed());
    println!("{result:?}\n");
}

fn task2() {
    let mut dictionary = env::current_dir().unwrap();
    dictionary.push("static/password_dictionary.txt");
    let hashes: Vec<&[u8]> = vec![
        b"1a7648bc484b3d9ed9e2226d223a6193d64e5e1fcacd97868adec665fe12b924",
        b"8c6976e5b5410415bde908bd4dee15dfb167a9c873fc4bb8a81f6f2ab448a918",
        b"48054a90032bf1348452fd74f3500ef8d2318d9b5582b07449b3b59db841eecd",
        b"09537eae89936399905661760584b19f6ff3af4bb807cee0bb663f64b07eea8e",
        b"e7798dc61be73b717402d76cbfaaef41c36c85c027a59abd74abbc8c8288bd4f",
        b"0f42bcbeedf89160a6cf7ccafe68080f2aafb73b3ef057df6b5e22f1294d0a10",
        b"13989fe9c124d4dfca4e2661dcf8449f49a76fb69f9725612a130622ff3f9bfb",
        b"d780c9776eb7d602c805af9ed7aa78225b36af0decb6be51045dcbfa661594a3",
        b"d2d03c10a4f2c361dbeff74dab0019264e37336f9ef04831943d0f07c0ad52c7",
        b"cbb05a10a2fc5cc96ce5da00a12acc54f594eadb85363de665f3e5dcb81d0e51",
    ];
    let now = Instant::now();
    let result = dictionary_attack(&dictionary, hashes);
    println!("DictionaryAttack took: {:?}", now.elapsed());
    println!("{result:?}");
}

fn main() {
    task1();
    task2();
}
