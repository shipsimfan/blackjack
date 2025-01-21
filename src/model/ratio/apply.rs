use crate::model::Ratio;

impl Ratio {
    /// Apply this [`Ratio`] to `value`
    pub fn apply(&self, value: u16) -> i32 {
        (value as i32 * self.numerator.get() as i32) / self.denominator.get() as i32
    }
}
