use criterion::{black_box, criterion_group, criterion_main, Criterion};
use p2::AVLTree; // Ensure this path is correct based on your crate structure
// use lib::AVLTree;

fn insert_benchmark(c: &mut Criterion) {
    c.bench_function("avl_tree_insert", |b| {
        b.iter_batched(
            || {
                let mut avl_tree = AVLTree::<i32>::new();
                let elements: Vec<i32> = (1..=100_000).collect();
                (avl_tree, elements)
            },
            |(mut avl_tree, elements)| {
                for element in elements {
                    avl_tree.insert(element);
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

fn search_benchmark(c: &mut Criterion) {
    c.bench_function("avl_tree_search", |b| {
        b.iter_batched(
            || {
                let mut avl_tree = AVLTree::<i32>::new();
                let elements: Vec<i32> = (1..=100_000).collect();
                elements.iter().for_each(|&x| avl_tree.insert(x));
                let search_elements: Vec<i32> = (1..=10_000).collect();
                (avl_tree, search_elements)
            },
            |(avl_tree, search_elements)| {
                for element in search_elements {
                    black_box(avl_tree.search(element));
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, insert_benchmark, search_benchmark);
criterion_main!(benches);
