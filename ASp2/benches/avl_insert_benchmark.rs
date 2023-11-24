use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ASp2::AVLTree; // Replace ASp2 with the name of your crate or module

#[cfg(feature = "debug_print")]
macro_rules! debug_println {
    ($($args:tt)*) => {
        println!($($args)*);
    };
}

#[cfg(not(feature = "debug_print"))]
macro_rules! debug_println {
    ($($args:tt)*) => {};
}

pub fn insert_benchmark(c: &mut Criterion) {
    let tree_sizes = [10_000, 40_000, 70_000, 100_000, 130_000];

    for &size in &tree_sizes {
        c.bench_function(&format!("insert {} elements", size), |b| {
            b.iter(|| {
                let mut tree = AVLTree::<i32>::new();
                for i in 0..size {
                    tree.insert(black_box(i));
                }
            });
        });
    }
}

criterion_group!(benches, insert_benchmark);
criterion_main!(benches);
