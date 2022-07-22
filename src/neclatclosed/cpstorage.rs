use std::collections::HashMap;

use super::bitvector::BitVector;
use roaring::RoaringBitmap;

#[derive(Default)]
pub struct CPStorage(HashMap<usize, Vec<RoaringBitmap>>);

impl CPStorage {
    pub fn new() -> CPStorage {
        CPStorage(HashMap::new())
    }

    pub fn insert_if_close(&mut self, itemset_bitvector: RoaringBitmap, support: usize) -> bool {
        let mut result = true;

        match self.0.get_mut(&support) {
            Some(list) => {
                let mut index = 0;

                for q in list.iter() {
                    if itemset_bitvector.len() >= q.len() {
                        break;
                    }
                    if itemset_bitvector.is_subset(q) {
                        result = false;
                        break;
                    }
                    index += 1;
                }

                if result {
                    list.insert(index, itemset_bitvector);
                }
            }
            None => {
                self.0.insert(support, vec![itemset_bitvector]);
            }
        }

        result
    }
}
