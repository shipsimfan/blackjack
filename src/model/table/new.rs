use crate::model::BlackjackTable;

impl BlackjackTable {
    /// Creates a new empty [`BlackjackTable`]
    pub fn new(max_players: usize) -> Self {
        let mut players = Vec::with_capacity(max_players);
        for _ in 0..max_players {
            players.push(None);
        }

        BlackjackTable {
            players: players.into_boxed_slice(),
        }
    }
}
