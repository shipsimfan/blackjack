use crate::model::{Card, Shoe};
use std::{num::NonZeroUsize, random::random};

/// The minimum depth a cut card can be, per deck
const MIN_CUT_CARD_PER_DECK: usize = 8;

/// The maximum depth a cut card can be, per deck
const MAX_CUT_CARD_PER_DECK: usize = 12;

/// Performs one step of a binary bucket sort
fn bucket_sort(
    cards: &mut [(Card, u32)],
    buffer: &mut [(Card, u32)],
    left: isize,
    right: isize,
    bit_position: i32,
) {
    fn bit(value: u32, bit: u32) -> bool {
        (value >> bit) & 1 != 0
    }

    if left >= right {
        return;
    }

    if bit_position < 0 {
        for i in left..=right {
            cards[i as usize].1 = random();
        }

        return bucket_sort(cards, buffer, left, right, 31);
    }

    let mut low = left;
    let mut high = right as isize;

    for i in left..=right {
        if bit(cards[i as usize].1, bit_position as u32) {
            buffer[high as usize] = cards[i as usize];
            high -= 1;
        } else {
            buffer[low as usize] = cards[i as usize];
            low += 1;
        }
    }

    cards[left as usize..=right as usize].copy_from_slice(&buffer[left as usize..=right as usize]);

    bucket_sort(cards, buffer, left, low - 1, bit_position - 1);
    bucket_sort(cards, buffer, low, right, bit_position - 1);
}

/// Perform a bucket sort on `input`
fn sort(cards: &mut [(Card, u32)]) {
    let mut buffer = Vec::with_capacity(cards.len());
    unsafe {
        buffer.set_len(cards.len());
    }
    bucket_sort(cards, &mut buffer, 0, cards.len() as isize - 1, 31);
}

impl Shoe {
    /// Shuffle this shoe
    pub fn shuffle(&mut self, set_cut_card: bool) {
        // Collect the cards to shuffle and assign random numbers
        let mut cards = Vec::with_capacity(self.size);
        cards.extend(self.cards.drain(..).map(|card| (card, random())));
        cards.extend(self.discard.drain(..).map(|card| (card, random())));

        // Sort the list by random numbers
        sort(&mut cards);

        // Set the cards
        self.cards.extend(cards.drain(..).map(|(card, _)| card));

        // Add rigged cards
        if self.rigged_cards.len() > 0 {
            self.size += self.rigged_cards.len();
            self.cards.extend(self.rigged_cards.drain(..));
        }

        // Select cut card
        if !set_cut_card {
            assert!(self.cut_card.is_none());
            return;
        }

        let min_cut_card = (self.size / 52) * MIN_CUT_CARD_PER_DECK;
        let max_cut_card = (self.size / 52) * MAX_CUT_CARD_PER_DECK;
        let diff = max_cut_card - min_cut_card;
        self.cut_card = NonZeroUsize::new(min_cut_card + (random::<usize>() % diff));
    }
}
