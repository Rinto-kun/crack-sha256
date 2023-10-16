use std::env;

use crack_sha256::{brute_force, brute_force_itertools, dictionary_attack};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const POSSIBLE_CHARS: &'static str = "abcdefghijklmnopqrstuvwxyz0123456789";

pub fn criterion_benchmark(c: &mut Criterion) {
    let hashes_task1= vec![
        "594e519ae499312b29433b7dd8a97ff068defcba9755b6d5d00e84c524d67b06",
        "ade5880f369fd9765fb6cffdf67b5d4dfb2cf650a49c848a0ce7be1c10e80b23",
        "83cf8b609de60036a8277bd0e96135751bbc07eb234256d4b65b893360651bf2",
        "0d335a3bea76dac4e3926d91c52d5bdd716bac2b16db8caf3fb6b7a58cbd92a7",
    ];

    c.bench_function("brute_force", |b| {
        b.iter(|| brute_force(hashes_task1.clone(), 4, POSSIBLE_CHARS.to_string()))
    });

    c.bench_function("brute_force_itertools", |b| {
        b.iter(|| brute_force_itertools(hashes_task1.clone(), 4, POSSIBLE_CHARS.to_string()))
    });

    let mut dictionary_path: std::path::PathBuf = env::current_dir().unwrap();
    dictionary_path.push("static/password_dictionary.txt");

    let hashes_task2 = vec![
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

    c.bench_function("dict_attack", |b| {
        b.iter(|| dictionary_attack(black_box(&dictionary_path), hashes_task2.clone()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
