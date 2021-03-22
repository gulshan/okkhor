#[macro_use]
extern crate criterion;
extern crate okkhor_lib;
extern crate rupantor;

use criterion::black_box;
use criterion::Criterion;
use okkhor_lib::parser::Phonetic;
use rupantor::avro::AvroPhonetic;

fn parse_benchmark(c: &mut Criterion) {
    c.bench_function("okkhor new", |b| b.iter(|| Phonetic::new()));
    c.bench_function("rupantor new", |b| b.iter(|| AvroPhonetic::new()));

    let input1 = "ami";
    c.bench_function("okkhor ami", |b| {
        let okkhor = Phonetic::new();
        b.iter(|| okkhor.convert(black_box(input1)))
    });
    c.bench_function("rupantor ami", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| avro.convert(black_box(input1)))
    });

    let input2 = "korrm";
    c.bench_function("okkhor kormo", |b| {
        let okkhor = Phonetic::new();
        b.iter(|| okkhor.convert(black_box(input2)))
    });
    c.bench_function("rupantor kormo", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| avro.convert(black_box(input2)))
    });

    let input3 = "bistarito";
    c.bench_function("okkhor bistarito", |b| {
        let okkhor = Phonetic::new();
        b.iter(|| okkhor.convert(black_box(input3)))
    });
    c.bench_function("rupantor bistarito", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| avro.convert(black_box(input3)))
    });

    let input4 = "ghoTOt``kc";
    c.bench_function("okkhor long word", |b| {
        let okkhor = Phonetic::new();
        b.iter(|| okkhor.convert(black_box(input4)))
    });
    c.bench_function("rupantor long word", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| avro.convert(black_box(input4)))
    });

    let input5 = "amar sOnar bangla";
    c.bench_function("okkhor sonar bangla", |b| {
        let okkhor = Phonetic::new();
        b.iter(|| okkhor.convert(black_box(input5)))
    });
    c.bench_function("rupantor sonar bangla", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| avro.convert(black_box(input5)))
    });

    let input6 = "ami banglay gan gai";
    c.bench_function("okkhor sentence 1", |b| {
        let okkhor = Phonetic::new();
        b.iter(|| okkhor.convert(black_box(input6)))
    });
    c.bench_function("rupantor sentence 1", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| avro.convert(black_box(input6)))
    });

    let input7 = "amader valObasa hoye gel ghas, kheye gel goru ar diye gelo ba^sh";
    c.bench_function("okkhor sentence 2", |b| {
        let okkhor = Phonetic::new();
        b.iter(|| okkhor.convert(black_box(input7)))
    });
    c.bench_function("rupantor sentence 2", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| avro.convert(black_box(input7)))
    });
}

criterion_group!(benches, parse_benchmark);
criterion_main!(benches);
