#![feature(test)]
extern crate test;

use test::{Bencher, black_box};

use rand::prelude::SliceRandom;

#[derive(Copy, Clone)]
enum Color {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(Copy, Clone)]
struct Poker(Color, &'static str);

const VALUES: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
];

#[cached::proc_macro::cached]
#[inline]
fn gen_all_poker() -> [Poker; 104] {
    #[inline]
    fn __inline() -> [[Poker; 13]; 4] {
        [Color::Spades, Color::Hearts, Color::Clubs, Color::Diamonds]
            .map(|c| VALUES.map(|v| Poker(c, v)))
    }
    let arr = [__inline(), __inline()];
    unsafe { std::mem::transmute(arr) }
}

#[bench]
fn bench(b: &mut Bencher) {
    b.iter(|| {
        let mut pokers = gen_all_poker();
        pokers.shuffle(&mut rand::thread_rng());
        let chunks = black_box(pokers.chunks(26));
        chunks
            .map(|ps| ps.to_vec())
            .collect::<Vec<Vec<Poker>>>()
            .into_iter()
            .for_each(|a| {
                black_box(a);
            });
    });
}

