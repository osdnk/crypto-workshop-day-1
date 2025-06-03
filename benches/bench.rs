use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use regev::*;
fn bench_keygen(c: &mut Criterion) {
    c.bench_function("keygen", |b| b.iter(|| {
        black_box(keygen::keygen());
    }));
}

fn bench_encrypt(c: &mut Criterion) {
    let (_, pk) = keygen::keygen();
    c.bench_function("encrypt", |b| b.iter(|| {
        black_box(encrypt::encrypt(&pk, 3));
    }));
}

fn bench_decrypt(c: &mut Criterion) {
    let (sk, pk) = keygen::keygen();
    let ct = encrypt::encrypt(&pk, 3);
    c.bench_function("decrypt", |b| b.iter(|| {
        black_box(decrypt::decrypt(&sk, &ct));
    }));
}

criterion_group!(benches, bench_keygen, bench_encrypt, bench_decrypt);
    criterion_main!(benches);