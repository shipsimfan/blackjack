use crate::RandomAI;
use blackjack::{
    client::{Move, AI},
    model::BlackjackTable,
};
use std::{num::NonZeroU16, random::random};

impl AI for RandomAI {
    fn make_move(&mut self, _: &BlackjackTable) -> Move {
        if let Some(chance) = self.hit_chance {
            let odds: u32 = random();

            if odds as u64 <= chance {
                return Move::Hit;
            }
        }

        Move::Stand
    }

    fn place_bet(&mut self, _: &BlackjackTable) -> NonZeroU16 {
        self.bet
    }
}
