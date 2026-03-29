//! Phase 2 Benchmarks - QI, 4D, GPU

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sf_qi::{AvoAnalysis, VpVsRatio, PoissonsRatio, Dhi, Gassmann};
use sf_4d::TimeLapseAnalysis;

fn bench_avo_gradient_intercept(c: &mut Criterion) {
    let angles = vec![0.0, 10.0, 20.0, 30.0, 40.0];
    let amplitudes = vec![2.0, 1.5, 1.0, 0.5, 0.0];
    let avo = AvoAnalysis::new(angles, amplitudes);
    
    c.bench_function("avo_gradient_intercept", |b| {
        b.iter(|| (black_box(&avo).gradient(), black_box(&avo).intercept()))
    });
}

fn bench_avo_full_analysis(c: &mut Criterion) {
    let angles = vec![0.0, 10.0, 20.0, 30.0, 40.0];
    let amplitudes = vec![2.0, 1.5, 1.0, 0.5, 0.0];
    let avo = AvoAnalysis::new(angles, amplitudes);
    
    c.bench_function("avo_full_analysis_classify", |b| {
        b.iter(|| {
            let _ = black_box(&avo).gradient();
            let _ = black_box(&avo).intercept();
            let _ = black_box(&avo).classify();
        })
    });
}

fn bench_elastic_parameters(c: &mut Criterion) {
    c.bench_function("vp_vs_ratio", |b| {
        b.iter(|| VpVsRatio::compute(black_box(2500.0), black_box(1200.0)))
    });
    
    c.bench_function("poissons_ratio", |b| {
        b.iter(|| PoissonsRatio::from_vp_vs(black_box(2500.0), black_box(1200.0)))
    });
}

fn bench_dhi_scoring(c: &mut Criterion) {
    c.bench_function("dhi_score_full", |b| {
        b.iter(|| {
            Dhi::score(
                black_box(2.3),
                black_box(0.12),
                black_box(-0.4),
                black_box(-0.4),
            )
        })
    });
}

fn bench_gassmann_substitution(c: &mut Criterion) {
    c.bench_function("gassmann_bulk_modulus", |b| {
        b.iter(|| {
            Gassmann::bulk_modulus(
                black_box(10.0),
                black_box(36.0),
                black_box(2.5),
                black_box(0.25),
            )
        })
    });
}

fn bench_4d_difference(c: &mut Criterion) {
    let base = vec![1.0; 10000];
    let monitor = vec![1.1; 10000];
    let analysis = TimeLapseAnalysis::new(base, monitor, 0.004);
    
    c.bench_function("4d_simple_difference", |b| {
        b.iter(|| black_box(&analysis).difference())
    });
}

fn bench_4d_rms_difference(c: &mut Criterion) {
    let base = vec![1.0; 10000];
    let monitor = vec![1.1; 10000];
    let analysis = TimeLapseAnalysis::new(base, monitor, 0.004);
    
    c.bench_function("4d_rms_difference", |b| {
        b.iter(|| black_box(&analysis).rms_difference())
    });
}

fn bench_4d_nrms(c: &mut Criterion) {
    let base = vec![1.0; 10000];
    let monitor = vec![1.1; 10000];
    let analysis = TimeLapseAnalysis::new(base, monitor, 0.004);
    
    c.bench_function("4d_nrms", |b| {
        b.iter(|| black_box(&analysis).nrms())
    });
}

fn bench_4d_large_volume(c: &mut Criterion) {
    let base = vec![1.0; 100000];
    let monitor = vec![1.05; 100000];
    let analysis = TimeLapseAnalysis::new(base, monitor, 0.004);
    
    c.bench_function("4d_large_volume_100k", |b| {
        b.iter(|| black_box(&analysis).rms_difference())
    });
}

criterion_group!(
    phase2_benches,
    bench_avo_gradient_intercept,
    bench_avo_full_analysis,
    bench_elastic_parameters,
    bench_dhi_scoring,
    bench_gassmann_substitution,
    bench_4d_difference,
    bench_4d_rms_difference,
    bench_4d_nrms,
    bench_4d_large_volume,
);
criterion_main!(phase2_benches);
