//! Phase 1 Benchmarks - ML & Attributes

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sf_ml::SyntheticTrainer;
use sf_attributes::{RmsAmplitude, MeanAmplitude, SeismicAttribute};

fn bench_ml_synthetic_data(c: &mut Criterion) {
    let mut trainer = SyntheticTrainer::new(42);
    c.bench_function("ml_synthetic_64x64", |b| {
        b.iter(|| trainer.generate_sample(black_box(64)))
    });
}

fn bench_ml_synthetic_batch(c: &mut Criterion) {
    let mut trainer = SyntheticTrainer::new(42);
    c.bench_function("ml_synthetic_batch_100", |b| {
        b.iter(|| trainer.generate_batch(black_box(100), black_box(64)))
    });
}

fn bench_rms_amplitude_1024(c: &mut Criterion) {
    let attr = RmsAmplitude;
    let trace = vec![1.0; 1024];
    c.bench_function("rms_amplitude_1024_samples", |b| {
        b.iter(|| attr.compute(black_box(&trace), black_box(64)))
    });
}

fn bench_rms_amplitude_10k(c: &mut Criterion) {
    let attr = RmsAmplitude;
    let trace = vec![1.0; 10000];
    c.bench_function("rms_amplitude_10k_samples", |b| {
        b.iter(|| attr.compute(black_box(&trace), black_box(128)))
    });
}

fn bench_mean_amplitude(c: &mut Criterion) {
    let attr = MeanAmplitude;
    let trace = vec![1.0; 1024];
    c.bench_function("mean_amplitude_1024_samples", |b| {
        b.iter(|| attr.compute(black_box(&trace), black_box(64)))
    });
}

fn bench_all_amplitude_attributes(c: &mut Criterion) {
    let attrs = sf_attributes::all_amplitude_attributes();
    let trace = vec![1.0; 512];
    
    c.bench_function("all_10_amplitude_attributes", |b| {
        b.iter(|| {
            for attr in &attrs {
                attr.compute(black_box(&trace), black_box(64));
            }
        })
    });
}

criterion_group!(
    phase1_benches,
    bench_ml_synthetic_data,
    bench_ml_synthetic_batch,
    bench_rms_amplitude_1024,
    bench_rms_amplitude_10k,
    bench_mean_amplitude,
    bench_all_amplitude_attributes,
);
criterion_main!(phase1_benches);
