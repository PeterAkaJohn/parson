use criterion::{criterion_group, criterion_main, Criterion};
use parson::Parson;
use std::{fs, hint::black_box, str::from_utf8};

fn get_json_mock_data_path() -> String {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    format!("{}/{}", crate_dir, "/tests/test_files/MOCK_DATA.json")
}

fn read_json_string(json_string: &str) {
    let parsed_json = Parson::parse_json(json_string);
    assert!(parsed_json.is_ok());
    let _ = parsed_json.unwrap();
}
fn read_json_bytes(bytes: &[u8]) {
    let parsed_json = Parson::parse_json_with_bytes(bytes);
    assert!(parsed_json.is_ok());
    let _ = parsed_json.unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let bytes = fs::read(get_json_mock_data_path()).unwrap();
    let json_string = from_utf8(&bytes).unwrap();
    c.bench_function("json_string", |b| {
        b.iter(|| read_json_string(black_box(json_string)))
    });
    c.bench_function("json_bytes", |b| {
        b.iter(|| read_json_bytes(black_box(&bytes)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
