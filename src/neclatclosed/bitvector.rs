use lazy_static::lazy_static;

lazy_static! {
    static ref TWO_POWER: [i64; 64] = {
        let mut v = [0; 64];
        for (i, item) in v.iter_mut().enumerate() {
            *item = 1 << i;
        }
        v
    };
}

#[derive(Default)]
pub struct BitVector {
    bits: Vec<i64>,
    pub cardinality: usize,
}

impl BitVector {
    pub fn new(itemset: &Vec<usize>, last: usize) -> BitVector {
        assert!(itemset.len() >= last);

        let length = itemset[0];

        let mut bits: Vec<i64> = vec![0; (length / 64) + 1];
        let cardinality = last;

        for item in itemset.iter().take(last) {
            bits[item / 64] |= TWO_POWER[item % 64];
        }

        BitVector { bits, cardinality }
    }

    pub fn is_subset(&self, other: &BitVector) -> bool {
        if self.cardinality >= other.cardinality {
            return false;
        }

        for i in 0..self.bits.len() {
            if self.bits[i] & (!other.bits[i]) != 0 {
                return false;
            }
        }
        true
    }
}
