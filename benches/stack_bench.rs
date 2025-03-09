use criterion::{black_box, Criterion, criterion_group, criterion_main};
use lock_free_structs::stack::LockFreeStack;

fn benchmark_push(c: &mut Criterion) {
    c.bench_function("lock_free_stack_push", |b| {
        b.iter(|| {
            let stack = LockFreeStack::new();
            stack.push(black_box(42));
        })
    });
}

fn benchmark_pop(c: &mut Criterion) {
    c.bench_function("lock_free_stack_pop", |b| {
        b.iter(|| {
            let stack = LockFreeStack::new();
            stack.push(42);
            black_box(stack.pop());
        })
    });
}

criterion_group!(benches, benchmark_push, benchmark_pop);
criterion_main!(benches);

