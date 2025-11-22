use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mobile_ai_orchestrator::reservoir::{encode_text, EchoStateNetwork};

fn bench_esn_update(c: &mut Criterion) {
    c.bench_function("esn_update", |b| {
        let mut esn = EchoStateNetwork::new(384, 1000, 100, 0.7, 0.95);
        let input = vec![0.5; 384];
        b.iter(|| {
            esn.update(black_box(&input));
        });
    });
}

fn bench_text_encoding(c: &mut Criterion) {
    c.bench_function("encode_text", |b| {
        let text = "This is a sample text for encoding benchmark";
        b.iter(|| {
            encode_text(black_box(text), 384);
        });
    });
}

fn bench_esn_output(c: &mut Criterion) {
    c.bench_function("esn_output", |b| {
        let esn = EchoStateNetwork::new(384, 1000, 100, 0.7, 0.95);
        b.iter(|| {
            esn.output();
        });
    });
}

fn bench_esn_creation(c: &mut Criterion) {
    c.bench_function("esn_creation", |b| {
        b.iter(|| {
            EchoStateNetwork::new(
                black_box(384),
                black_box(1000),
                black_box(100),
                black_box(0.7),
                black_box(0.95),
            );
        });
    });
}

criterion_group!(
    benches,
    bench_esn_update,
    bench_text_encoding,
    bench_esn_output,
    bench_esn_creation
);
criterion_main!(benches);
