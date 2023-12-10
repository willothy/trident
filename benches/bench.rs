use criterion::{black_box, criterion_group, criterion_main, Criterion};
use trie::Trie;

fn searching(c: &mut Criterion) {
    let mut trie = Trie::new();

    let words = parity_wordlist::WORDS;

    // Populate trie
    let count = words.len();
    for word in words {
        trie.insert(word);
    }

    c.bench_function("iterative", |b| {
        let key = format!("{}", words[rand::random::<usize>() % count]);
        b.iter(|| {
            trie.search(black_box(&key)) // ??
        })
    });
    c.bench_function("recursive 1", |b| {
        let mut key = format!("{}", words[rand::random::<usize>() % count]);
        b.iter(|| {
            trie.search_rec(black_box(key.as_mut())) // ??
        })
    });
    c.bench_function("recursive 2", |b| {
        let key = format!("{}", words[rand::random::<usize>() % count]);
        b.iter(|| {
            trie.search_rec2(black_box(&key)) // ??
        })
    });
}

criterion_group!(benches, searching);
criterion_main!(benches);
