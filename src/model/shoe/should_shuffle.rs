use crate::model::Shoe;

impl Shoe {
    /// Should this shoe be shuffled?
    pub fn should_shuffle(&self) -> bool {
        self.cut_card.is_none()
    }
}
