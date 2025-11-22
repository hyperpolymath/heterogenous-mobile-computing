use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mobile_ai_orchestrator::{Orchestrator, Query};

fn bench_simple_query(c: &mut Criterion) {
    c.bench_function("orchestrator_simple_query", |b| {
        let mut orch = Orchestrator::new();
        b.iter(|| {
            let query = Query::new(black_box("How do I iterate HashMap?"));
            orch.process(query).ok()
        });
    });
}

fn bench_complex_query(c: &mut Criterion) {
    c.bench_function("orchestrator_complex_query", |b| {
        let mut orch = Orchestrator::new();
        b.iter(|| {
            let query = Query::new(black_box(
                "Can you prove this theorem using formal verification methods?",
            ));
            // This will route to Remote and fail without network, but we benchmark the routing
            orch.process(query).ok()
        });
    });
}

fn bench_context_switching(c: &mut Criterion) {
    c.bench_function("context_project_switch", |b| {
        let mut orch = Orchestrator::new();
        b.iter(|| {
            orch.switch_project(black_box("project-1"));
            orch.switch_project(black_box("project-2"));
        });
    });
}

fn bench_conversation_history(c: &mut Criterion) {
    c.bench_function("add_conversation_turn", |b| {
        let mut orch = Orchestrator::new();
        b.iter(|| {
            let query = Query::new(black_box("test query"));
            orch.process(query).ok();
        });
    });
}

criterion_group!(
    benches,
    bench_simple_query,
    bench_complex_query,
    bench_context_switching,
    bench_conversation_history
);
criterion_main!(benches);
