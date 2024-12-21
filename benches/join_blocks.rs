use criterion::{criterion_group, criterion_main, Criterion};
use text_block_layout::{self, Block};

fn join_blocks(c: &mut Criterion) {
    let right: Block = "foo".into();
    let bottom: Block = "bar".into();

    c.bench_function("join_blocks", |b| {
        b.iter(|| {
            let mut block = Block::empty();
            for _ in 0..50 {
                block = block.beside_center_top(&right).stack_center_left(&bottom);
            }
        })
    });
}

criterion_group!(benches, join_blocks);
criterion_main!(benches);
