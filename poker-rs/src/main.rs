use rand::prelude::SliceRandom;
use std::fmt::{Debug, Display, Formatter};

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

fn main() {
    let mut pokers = gen_all_poker();
    pokers.shuffle(&mut rand::thread_rng());
    let chunks = pokers.chunks(26);
    chunks
        .map(|ps| ps.to_vec())
        .collect::<Vec<Vec<Poker>>>()
        .into_iter()
        .enumerate()
        .for_each(|(n, pokers)| println!("player {}: {:?}", n, pokers));
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Spades => write!(f, "♠"),
            Color::Hearts => write!(f, "❤️"),
            Color::Clubs => write!(f, "♣"),
            Color::Diamonds => write!(f, "♦"),
        }
    }
}

impl Display for Poker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}", self.0, self.1)
    }
}

impl Debug for Poker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
