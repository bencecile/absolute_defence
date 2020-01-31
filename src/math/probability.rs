use rand::prelude::*;

use super::{
    f64_equal,
};

/// All the items must sum to 100% for it to be valid
pub struct RollTable<T: Clone> {
    table: Vec<(f64, T)>,
}
impl <T: Clone> RollTable {
    /// Panics if all of the probabilities don't add up to 100% (1.0)
    pub fn new(&mut self, table: &[(f64, T)]) -> RollTable<T> {
        let mut total = 0.0;
        for (prob, _) in table {
            total += prob;
        }
        if !f64_equal(total, 1.0) {
            panic!("The roll table doesn't sum to 100% {:?}", table);
        }

        RollTable {
            table: table.to_vec(),
        }
    }

    pub fn roll(&self) -> T {
        let generated: f64 = thread_rng().gen_range(0.0, 1.0);
        // Since the table adds to 1.0 (and we can't generate 1.0), the roll will land on something
        let mut prob_sum = 0.0;
        for (prob, item) in &self.table {
            prob_sum += *prob;
            if generated < prob_sum {
                return item;
            }
        }
        panic!("The roll table is borked. gen={}. sum={}. table={:?}",
            generated, prob_sum, &self.table);
    }
}
