#[macro_use]
extern crate criterion;
extern crate okkhor;
extern crate rupantor;

use criterion::black_box;
use criterion::Criterion;
use okkhor::parser::Parser;
use rupantor::avro::AvroPhonetic;

#[cfg(feature = "regex")]
mod regex;

fn parse_benchmark(c: &mut Criterion) {
    c.bench_function("okkhor new", |b| b.iter(Parser::new_phonetic));

    #[cfg(feature = "regex")]
    c.bench_function("rupantor new", |b| b.iter(AvroPhonetic::new));

    let input1 = "ami";
    c.bench_function("okkhor ami", |b| {
        let okkhor = Parser::new_phonetic();
        b.iter(|| okkhor.convert(black_box(input1)))
    });

    #[cfg(feature = "regex")]
    c.bench_function("rupantor ami", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| avro.convert(black_box(input1)))
    });

    let input2 = "korrm";
    c.bench_function("okkhor kormo", |b| {
        let okkhor = Parser::new_phonetic();
        b.iter(|| okkhor.convert(black_box(input2)))
    });

    #[cfg(feature = "regex")]
    c.bench_function("rupantor kormo", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| avro.convert(black_box(input2)))
    });

    let input3 = "bistarito";
    c.bench_function("okkhor bistarito", |b| {
        let okkhor = Parser::new_phonetic();
        b.iter(|| okkhor.convert(black_box(input3)))
    });

    #[cfg(feature = "regex")]
    c.bench_function("rupantor bistarito", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| avro.convert(black_box(input3)))
    });

    let input4 = "ghoTOt``kc";
    c.bench_function("okkhor long word", |b| {
        let okkhor = Parser::new_phonetic();
        b.iter(|| okkhor.convert(black_box(input4)))
    });

    #[cfg(feature = "regex")]
    c.bench_function("rupantor long word", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| avro.convert(black_box(input4)))
    });

    let input5 = "amar sOnar bangla";
    c.bench_function("okkhor sonar bangla", |b| {
        let okkhor = Parser::new_phonetic();
        b.iter(|| okkhor.convert(black_box(input5)))
    });

    #[cfg(feature = "regex")]
    c.bench_function("rupantor sonar bangla", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| avro.convert(black_box(input5)))
    });

    let input6 = "ami banglay gan gai";
    c.bench_function("okkhor sentence 1", |b| {
        let okkhor = Parser::new_phonetic();
        b.iter(|| okkhor.convert(black_box(input6)))
    });

    #[cfg(feature = "regex")]
    c.bench_function("rupantor sentence 1", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| avro.convert(black_box(input6)))
    });

    let input7 = "amader valObasa hoye gel ghas, kheye gel goru ar diye gelo ba^sh";
    c.bench_function("okkhor sentence 2", |b| {
        let okkhor = Parser::new_phonetic();
        b.iter(|| okkhor.convert(black_box(input7)))
    });

    #[cfg(feature = "regex")]
    c.bench_function("rupantor sentence 2", |b| {
        let avro = AvroPhonetic::new();
        b.iter(|| avro.convert(black_box(input7)))
    });
}

#[cfg(feature = "regex")]
fn parse_regex_benchmark(c: &mut Criterion) {
    let input1 = "a";
    c.bench_function("okkhor regex a", |b| {
        let okkhor = Parser::new_regex();
        b.iter(|| okkhor.convert_regex(black_box(input1)));
    });
    c.bench_function("okkhor regex a buf", |b| {
        let okkhor = Parser::new_regex();
        let mut buffer = String::with_capacity(512);
        b.iter(|| okkhor.convert_regex_into(black_box(input1), &mut buffer));
    });
    c.bench_function("riti regex a", |b| {
        b.iter(|| regex::parse(black_box(input1)));
    });
    c.bench_function("riti regex a buf", |b| {
        let mut buffer = String::with_capacity(512);
        b.iter(|| regex::parse_buf(black_box(input1), &mut buffer));
    });

    let input2 = "bistari";
    c.bench_function("okkhor regex bistari", |b| {
        let okkhor = Parser::new_regex();
        b.iter(|| okkhor.convert_regex(black_box(input2)));
    });
    c.bench_function("okkhor regex bistari buf", |b| {
        let okkhor = Parser::new_regex();
        let mut buffer = String::with_capacity(512);
        b.iter(|| okkhor.convert_regex_into(black_box(input2), &mut buffer));
    });
    c.bench_function("riti regex bistari", |b| {
        b.iter(|| regex::parse(black_box(input2)));
    });
    c.bench_function("riti regex bistari buf", |b| {
        let mut buffer = String::with_capacity(512);
        b.iter(|| regex::parse_buf(black_box(input2), &mut buffer));
    });

    let input3 = "arO";
    c.bench_function("okkhor regex arO", |b| {
        let okkhor = Parser::new_regex();
        b.iter(|| okkhor.convert_regex(black_box(input3)));
    });
    c.bench_function("okkhor regex arO buf", |b| {
        let okkhor = Parser::new_regex();
        let mut buffer = String::with_capacity(512);
        b.iter(|| okkhor.convert_regex_into(black_box(input3), &mut buffer));
    });
    c.bench_function("riti regex arO", |b| {
        b.iter(|| regex::parse(black_box(input3)));
    });
    c.bench_function("riti regex arO buf", |b| {
        let mut buffer = String::with_capacity(512);
        b.iter(|| regex::parse_buf(black_box(input3), &mut buffer));
    });

    let input4 = "kkhet";
    c.bench_function("okkhor regex kkhet", |b| {
        let okkhor = Parser::new_regex();
        b.iter(|| okkhor.convert_regex(black_box(input4)));
    });
    c.bench_function("okkhor regex kkhet buf", |b| {
        let okkhor = Parser::new_regex();
        let mut buffer = String::with_capacity(512);
        b.iter(|| okkhor.convert_regex_into(black_box(input4), &mut buffer));
    });
    c.bench_function("riti regex kkhet", |b| {
        b.iter(|| regex::parse(black_box(input4)));
    });
    c.bench_function("riti regex kkhet buf", |b| {
        let mut buffer = String::with_capacity(512);
        b.iter(|| regex::parse_buf(black_box(input4), &mut buffer));
    });
}

#[cfg(feature = "regex")]
criterion_group!(benches, parse_regex_benchmark);
#[cfg(not(feature = "regex"))]
criterion_group!(benches, parse_benchmark);
criterion_main!(benches);
