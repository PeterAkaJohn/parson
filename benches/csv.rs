use criterion::{criterion_group, criterion_main, Criterion};
use parson::Parson;
use std::{fs, hint::black_box, str::from_utf8};

fn get_csv_mock_data_path() -> String {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    format!("{}/{}", crate_dir, "/tests/test_files/MOCK_DATA.csv")
}

fn read_csv_string(csv_string: &str) {
    let parsed_csv = Parson::parse_csv(csv_string);
    assert!(parsed_csv.is_ok());
    let _ = parsed_csv.unwrap();
}
fn read_csv_bytes(bytes: &[u8]) {
    let parsed_csv = Parson::parse_csv_with_bytes(bytes);
    assert!(parsed_csv.is_ok());
    let _ = parsed_csv.unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let bytes = fs::read(get_csv_mock_data_path()).unwrap();
    let csv_string = from_utf8(&bytes).unwrap();
    c.bench_function("csv_string", |b| {
        b.iter(|| read_csv_string(black_box(csv_string)))
    });
    c.bench_function("csv_bytes", |b| {
        b.iter(|| read_csv_bytes(black_box(&bytes)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
