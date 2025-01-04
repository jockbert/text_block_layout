//! Test bench for redering the text of a text block.

use criterion::{criterion_group, criterion_main, Criterion};
use text_block_layout::{self, Block};

fn rendering(c: &mut Criterion) {
    let texts = vec![
        "foo",
        "bar",
        "baz",
        "lorem ipsum 0",
        "lorem ipsum 1",
        "lorem ipsum 2",
        "lorem ipsum 3",
        "lorem ipsum 4",
        "lorem ipsum 5",
        "lorem ipsum 6",
        "lorem ipsum 7",
        "lorem ipsum 8",
        "lorem ipsum 9",
        "lorem ipsum 10",
        "lorem ipsum 11",
        "lorem ipsum 12",
        "lorem ipsum 13",
        "lorem ipsum 14",
        "lorem ipsum 15",
        "lorem ipsum 16",
        "lorem ipsum 17",
        "lorem ipsum 18",
        "lorem ipsum 19",
        "lorem ipsum 20",
        "lorem ipsum 21",
        "lorem ipsum 22",
        "lorem ipsum 23",
        "lorem ipsum 24",
        "lorem ipsum 25",
        "lorem ipsum 26",
        "lorem ipsum 27",
        "lorem ipsum 28",
        "lorem ipsum 29",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let block = Block::empty().add_multiple_texts(&texts);

    c.bench_function("rendering", |b| {
        b.iter(|| {
            block.to_string();
        })
    });
}

criterion_group!(benches, rendering);
criterion_main!(benches);
