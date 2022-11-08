use std::{collections::HashSet, env, time::Instant, vec};

use rustpg::{brute_force, dict_attack};

fn task1() {
    // let hashes: HashSet<&str> = HashSet::from([
    //     "594e519ae499312b29433b7dd8a97ff068defcba9755b6d5d00e84c524d67b06",
    //     "ade5880f369fd9765fb6cffdf67b5d4dfb2cf650a49c848a0ce7be1c10e80b23",
    //     "83cf8b609de60036a8277bd0e96135751bbc07eb234256d4b65b893360651bf2",
    //     "0d335a3bea76dac4e3926d91c52d5bdd716bac2b16db8caf3fb6b7a58cbd92a7",
    // ]);
    let hash_1 =
        hex::decode("594e519ae499312b29433b7dd8a97ff068defcba9755b6d5d00e84c524d67b06").unwrap();
    let hash_2 =
        hex::decode("ade5880f369fd9765fb6cffdf67b5d4dfb2cf650a49c848a0ce7be1c10e80b23").unwrap();
    let hash_3 =
        hex::decode("83cf8b609de60036a8277bd0e96135751bbc07eb234256d4b65b893360651bf2").unwrap();
    let hash_4 =
        hex::decode("0d335a3bea76dac4e3926d91c52d5bdd716bac2b16db8caf3fb6b7a58cbd92a7").unwrap();

    let hashes: Vec<&[u8]> = vec![
        &hash_1[..],
        &hash_2[..],
        &hash_3[..],
        &hash_4[..],
        // "ade5880f369fd9765fb6cffdf67b5d4dfb2cf650a49c848a0ce7be1c10e80b23",
        // "83cf8b609de60036a8277bd0e96135751bbc07eb234256d4b65b893360651bf2",
        // "0d335a3bea76dac4e3926d91c52d5bdd716bac2b16db8caf3fb6b7a58cbd92a7",
    ];

    let now = Instant::now();
    let result = brute_force(hashes, 4);
    println!("{result:?}");
    println!("{:?}", now.elapsed());
}

fn task2() {
    let dictionary = env::current_dir().unwrap();
    let dictionary = dictionary.to_str().unwrap().to_owned() + "/static/password_dictionary.txt";
    let hashes = vec![
        "1a7648bc484b3d9ed9e2226d223a6193d64e5e1fcacd97868adec665fe12b924",
        "8c6976e5b5410415bde908bd4dee15dfb167a9c873fc4bb8a81f6f2ab448a918",
        "48054a90032bf1348452fd74f3500ef8d2318d9b5582b07449b3b59db841eecd",
        "09537eae89936399905661760584b19f6ff3af4bb807cee0bb663f64b07eea8e",
        "e7798dc61be73b717402d76cbfaaef41c36c85c027a59abd74abbc8c8288bd4f",
        "0f42bcbeedf89160a6cf7ccafe68080f2aafb73b3ef057df6b5e22f1294d0a10",
        "13989fe9c124d4dfca4e2661dcf8449f49a76fb69f9725612a130622ff3f9bfb",
        "d780c9776eb7d602c805af9ed7aa78225b36af0decb6be51045dcbfa661594a3",
        "d2d03c10a4f2c361dbeff74dab0019264e37336f9ef04831943d0f07c0ad52c7",
        "cbb05a10a2fc5cc96ce5da00a12acc54f594eadb85363de665f3e5dcb81d0e51",
    ];

    let now = Instant::now();
    let result = dict_attack(&dictionary, hashes);
    println!("{result:?}");
    println!("Elapsed: {:?}", now.elapsed());
}

// fn task1_(){
//         let hashes = HashSet::from_iter([
//         b"594e519ae499312b29433b7dd8a97ff068defcba9755b6d5d00e84c524d67b06",
//         b"ade5880f369fd9765fb6cffdf67b5d4dfb2cf650a49c848a0ce7be1c10e80b23",
//         b"83cf8b609de60036a8277bd0e96135751bbc07eb234256d4b65b893360651bf2",
//         b"0d335a3bea76dac4e3926d91c52d5bdd716bac2b16db8caf3fb6b7a58cbd92a7",
//     ]);

//     let start = Instant::now();

//     let result = brute_force(hashes, 4);

//     println!("elapsed: {:?}", start.elapsed());

//     println!("{result:?}");
// }

fn main() {
    task1();
    // task1_();
    // task2();
}
