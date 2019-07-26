#[macro_use]
extern crate criterion;
extern crate okkhor_lib;
extern crate rupantor;

use criterion::black_box;
use criterion::Criterion;
use okkhor_lib::parser::parse;
use rupantor::avro::AvroPhonetic;

fn parse_benchmark(c: &mut Criterion) {
    c.bench_function("okkhor ami", |b| b.iter(|| parse(black_box("ami"))));
    c.bench_function("rupantor ami", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| avro.convert(black_box("ami")))
    });

    c.bench_function("okkhor kormo", |b| b.iter(|| parse(black_box("korrm"))));
    c.bench_function("rupantor kormo", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| avro.convert(black_box("korrm")))
    });

    c.bench_function("okkhor bistarito", |b| {
        b.iter(|| parse(black_box("bistarito")))
    });
    c.bench_function("rupantor bistarito", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| avro.convert(black_box("bistarito")))
    });

    c.bench_function("okkhor long word", |b| {
        b.iter(|| parse(black_box("ghoTOt``kc")))
    });
    c.bench_function("rupantor long word", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| avro.convert(black_box("ghoTOt``kc")))
    });

    c.bench_function("okkhor sentence 1", |b| {
        b.iter(|| parse(black_box("ami banglay gan gai")))
    });
    c.bench_function("rupantor sentence 1", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| avro.convert(black_box("ami banglay gan gai")))
    });

    c.bench_function("okkhor sentence 2", |b| {
        b.iter(|| {
            parse(black_box(
                "amader valObasa hoye gel ghas, kheye gel goru ar diye gelo ba^sh",
            ))
        })
    });
    c.bench_function("rupantor sentence 2", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| {
            avro.convert(black_box(
                "amader valObasa hoye gel ghas, kheye gel goru ar diye gelo ba^sh",
            ))
        })
    });
}

criterion_group!(benches, parse_benchmark);
criterion_main!(benches);
