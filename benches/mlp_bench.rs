use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mobile_ai_orchestrator::mlp::MLP;

fn bench_mlp_forward_small(c: &mut Criterion) {
    c.bench_function("mlp_forward_small", |b| {
        let mlp = MLP::new(10, vec![20], 3);
        let input = vec![0.5; 10];
        b.iter(|| {
            mlp.forward(black_box(&input));
        });
    });
}

fn bench_mlp_forward_medium(c: &mut Criterion) {
    c.bench_function("mlp_forward_medium", |b| {
        let mlp = MLP::new(384, vec![100, 50], 3);
        let input = vec![0.5; 384];
        b.iter(|| {
            mlp.forward(black_box(&input));
        });
    });
}

fn bench_mlp_forward_large(c: &mut Criterion) {
    c.bench_function("mlp_forward_large", |b| {
        let mlp = MLP::new(1000, vec![500, 250, 100], 10);
        let input = vec![0.5; 1000];
        b.iter(|| {
            mlp.forward(black_box(&input));
        });
    });
}

fn bench_softmax(c: &mut Criterion) {
    c.bench_function("softmax", |b| {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        b.iter(|| {
            MLP::softmax(black_box(&values));
        });
    });
}

fn bench_argmax(c: &mut Criterion) {
    c.bench_function("argmax", |b| {
        let values = vec![0.1, 0.8, 0.3, 0.5, 0.2];
        b.iter(|| {
            MLP::argmax(black_box(&values));
        });
    });
}

criterion_group!(
    benches,
    bench_mlp_forward_small,
    bench_mlp_forward_medium,
    bench_mlp_forward_large,
    bench_softmax,
    bench_argmax
);
criterion_main!(benches);
