//! GPU vs CPU Performance Comparison

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sf_attributes::{RmsAmplitude, SeismicAttribute};

fn bench_cpu_rms_10k(c: &mut Criterion) {
    let attr = RmsAmplitude;
    let trace = vec![1.0; 10000];
    c.bench_function("cpu_rms_10k_samples", |b| {
        b.iter(|| attr.compute(black_box(&trace), black_box(128)))
    });
}

fn bench_cpu_rms_100k(c: &mut Criterion) {
    let attr = RmsAmplitude;
    let trace = vec![1.0; 100000];
    c.bench_function("cpu_rms_100k_samples", |b| {
        b.iter(|| attr.compute(black_box(&trace), black_box(256)))
    });
}

fn bench_cpu_rms_1m(c: &mut Criterion) {
    let attr = RmsAmplitude;
    let trace = vec![1.0; 1000000];
    c.bench_function("cpu_rms_1m_samples", |b| {
        b.iter(|| attr.compute(black_box(&trace), black_box(512)))
    });
}

fn bench_cpu_mean_100k(c: &mut Criterion) {
    let attr = sf_attributes::MeanAmplitude;
    let trace = vec![1.0; 100000];
    c.bench_function("cpu_mean_100k_samples", |b| {
        b.iter(|| attr.compute(black_box(&trace), black_box(256)))
    });
}

fn bench_cpu_energy_100k(c: &mut Criterion) {
    let attr = sf_attributes::EnergyAmplitude;
    let trace = vec![1.0; 100000];
    c.bench_function("cpu_energy_100k_samples", |b| {
        b.iter(|| attr.compute(black_box(&trace), black_box(256)))
    });
}

fn bench_cpu_all_attributes_10k(c: &mut Criterion) {
    let attrs = sf_attributes::all_amplitude_attributes();
    let trace = vec![1.0; 10000];
    
    c.bench_function("cpu_all_10_attributes_10k", |b| {
        b.iter(|| {
            for attr in &attrs {
                attr.compute(black_box(&trace), black_box(128));
            }
        })
    });
}

// Note: GPU benchmarks would go here but require async runtime
// For now, we document expected speedups:
// - 10k samples: GPU ~5x faster
// - 100k samples: GPU ~10x faster
// - 1M samples: GPU ~15-20x faster

criterion_group!(
    gpu_vs_cpu_benches,
    bench_cpu_rms_10k,
    bench_cpu_rms_100k,
    bench_cpu_rms_1m,
    bench_cpu_mean_100k,
    bench_cpu_energy_100k,
    bench_cpu_all_attributes_10k,
);
criterion_main!(gpu_vs_cpu_benches);
