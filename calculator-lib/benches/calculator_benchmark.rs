use criterion::{black_box, criterion_group, criterion_main, Criterion};
use calculator_lib::{naive, optimized, Calculator};
use rand::Rng;

fn generate_data(n: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(-1000.0..1000.0)).collect()
}

fn naive_benchmark(c: &mut Criterion) {
    let data = generate_data(1_000_000);
    
    c.bench_function("naive appending", |b| {
        b.iter(|| {
            let mut calc = naive();
            calc.append(&data);
            black_box(&calc);
        })
    });

    let mut calc = naive();
    calc.append(&data);
    c.bench_function("naive calculating", |b| {
        b.iter(|| {
            black_box(calc.calculate_stats(8));
        })
    });

    
    c.bench_function("optimized appending", |b| {
        b.iter(|| {
            let mut calc = optimized();
            calc.append(&data);
            black_box(&calc);
        })
    });

    let mut calc = optimized();
    calc.append(&data);
    c.bench_function("optimized calculating", |b| {
        b.iter(|| {
            black_box(calc.calculate_stats(8));
        })
    });}

fn optimized_benchmark(c: &mut Criterion) {
    let data = generate_data(1_000_000);

}

criterion_group!(benches, naive_benchmark);
criterion_main!(benches);