use crate::model::Player;

impl Player {
    /// Payout this player a certain `amount`
    pub fn payout(&mut self, amount: i32) {
        match &mut self.last_round_earnings {
            Some(round_amount) => *round_amount += amount,
            None => self.last_round_earnings = Some(amount),
        }
        self.total_earnings += amount as i64;
    }
}
