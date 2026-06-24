use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use engine::{Engine, Side};
use uuid::Uuid;

// ── helpers ──────────────────────────────────────────────────────────────────

fn fresh_engine() -> Engine {
    Engine::new(None)
}

/// Fill the book with `n` resting bid orders spread across ticks 900..=999
/// and `n` ask orders across ticks 1001..=1100 so market orders can match.
fn prefilled_engine(n: u64) -> Engine {
    let mut e = fresh_engine();
    for i in 0..n {
        let tick = 900 + (i % 100);
        let _ = e.submit_limit_order(Uuid::new_v4(), Side::Bid, tick, 10);
    }
    for i in 0..n {
        let tick = 1001 + (i % 100);
        let _ = e.submit_limit_order(Uuid::new_v4(), Side::Ask, tick, 10);
    }
    e
}

// ── benchmarks ───────────────────────────────────────────────────────────────

/// How many resting limit orders can the engine accept per second.
fn bench_limit_order_tps(c: &mut Criterion) {
    let mut group = c.benchmark_group("limit_order_tps");

    for &n in &[1_000u64, 10_000, 100_000] {
        group.throughput(Throughput::Elements(n));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| {
                let mut engine = fresh_engine();
                // alternate bid/ask so orders rest rather than match
                for i in 0..n {
                    let (side, tick) = if i % 2 == 0 {
                        (Side::Bid, 900 + (i % 100))
                    } else {
                        (Side::Ask, 1001 + (i % 100))
                    };
                    let _ = engine.submit_limit_order(Uuid::new_v4(), side, tick, 1);
                }
            });
        });
    }

    group.finish();
}

/// How many market orders can the engine match per second against a live book.
fn bench_market_order_tps(c: &mut Criterion) {
    let mut group = c.benchmark_group("market_order_tps");

    for &n in &[1_000u64, 10_000] {
        group.throughput(Throughput::Elements(n));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter_batched(
                || prefilled_engine(n),
                |mut engine| {
                    for _ in 0..n {
                        let _ = engine.submit_market_order(Uuid::new_v4(), Side::Bid, 1);
                    }
                },
                criterion::BatchSize::LargeInput,
            );
        });
    }

    group.finish();
}

/// Mixed workload: 60 % limit, 20 % market, 20 % cancel.
fn bench_mixed_tps(c: &mut Criterion) {
    let n: u64 = 10_000;
    let mut group = c.benchmark_group("mixed_tps");
    group.throughput(Throughput::Elements(n));

    group.bench_function("60pct_limit_20pct_market_20pct_cancel", |b| {
        b.iter(|| {
            let mut engine = fresh_engine();
            let mut resting: Vec<(usize, Side)> = Vec::new();
            let mut cancel_idx = 0usize;

            for i in 0..n {
                match i % 5 {
                    // 60 % limit
                    0 | 1 | 2 => {
                        let (side, tick) = if i % 2 == 0 {
                            (Side::Bid, 900 + (i % 100))
                        } else {
                            (Side::Ask, 1001 + (i % 100))
                        };
                        if let Ok(report) =
                            engine.submit_limit_order(Uuid::new_v4(), side, tick, 5)
                        {
                            if let Some(idx) = report.resting_order_idx {
                                resting.push((idx, side));
                            }
                        }
                    }
                    // 20 % market
                    3 => {
                        let _ = engine.submit_market_order(Uuid::new_v4(), Side::Bid, 1);
                    }
                    // 20 % cancel
                    _ => {
                        if cancel_idx < resting.len() {
                            let (idx, _) = resting[cancel_idx];
                            let _ = engine.cancel_order(idx);
                            cancel_idx += 1;
                        }
                    }
                }
            }
        });
    });

    group.finish();
}

/// Raw BitMap set + first_highest_tick lookup — the hot path inside the book.
fn bench_bitmap_tps(c: &mut Criterion) {
    use engine::Engine;
    let n: u64 = 100_000;
    let mut group = c.benchmark_group("bitmap_via_limit_orders");
    group.throughput(Throughput::Elements(n));

    group.bench_function("set_and_best_bid_lookup", |b| {
        b.iter(|| {
            let mut engine = Engine::new(None);
            for i in 0..n {
                let tick = 500 + (i % 500);
                let _ = engine.submit_limit_order(Uuid::new_v4(), Side::Bid, tick, 1);
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_limit_order_tps,
    bench_market_order_tps,
    bench_mixed_tps,
    bench_bitmap_tps,
);
criterion_main!(benches);
