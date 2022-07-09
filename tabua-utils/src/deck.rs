use rand::prelude::SliceRandom;
use rand::Rng;

pub trait Deck {
    type TokenType;

    fn shuffle(&mut self);

    fn draw(&mut self) -> Option<Self::TokenType>;
    fn draw_random(&mut self) -> Option<Self::TokenType>;

    fn peek(&self, count: usize) -> Vec<&Self::TokenType>;
    fn peek_top(&self) -> Option<&Self::TokenType>;

    fn put_top(&mut self, token: Self::TokenType);
    fn put_bottom(&mut self, token: Self::TokenType);
    fn put_random(&mut self, token: Self::TokenType);
}

impl<T> Deck for Vec<T> {
    type TokenType = T;

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.as_mut_slice().shuffle(&mut rng);
    }

    fn draw(&mut self) -> Option<Self::TokenType> {
        self.pop()
    }

    fn draw_random(&mut self) -> Option<Self::TokenType> {
        if self.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let n: usize = rng.gen_range(0..self.len());
        Some(self.remove(n))
    }

    fn peek(&self, n: usize) -> Vec<&Self::TokenType> {
        self.iter().rev().take(n).collect()
    }

    fn peek_top(&self) -> Option<&Self::TokenType> {
        self.last()
    }

    fn put_top(&mut self, token: Self::TokenType) {
        self.push(token)
    }

    fn put_bottom(&mut self, token: Self::TokenType) {
        self.insert(0, token)
    }

    fn put_random(&mut self, token: Self::TokenType) {
        let mut rng = rand::thread_rng();
        let n: usize = rng.gen_range(0..self.len());
        self.insert(n, token);
    }
}
