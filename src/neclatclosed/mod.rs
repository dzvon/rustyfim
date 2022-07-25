//! NEclatClosed algorithm implementation.

mod bitvector;
mod cpstorage;
mod itemset;

use self::cpstorage::CPStorage;
use bitvector::BitVector;
use roaring::RoaringBitmap;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub use self::itemset::ItemSet;

#[derive(Default)]
pub struct NEclatClosed {
    n_items: usize,     // number of items
    min_support: usize, // minimum support threshold
    items: Vec<Item>,   // list of items sorted by count

    n_transactions: usize, // number of transactions

    itemset_x: Vec<usize>, // the current itemset
    itemset_x_len: usize,  // the length of the current itemset

    // A map containing the tidset (i.e. cover) of each item represented as a bitvector.
    item_tids: HashMap<usize, RoaringBitmap, fnv::FnvBuildHasher>,

    nl_root: TreeNode, // root of the tree

    cp_storage: CPStorage, // storage for closed patterns

    closed_itemsets: Vec<ItemSet>, // list of closed itemsets
}

#[derive(PartialEq, Eq, Debug)]
struct Item {
    index: usize,
    num: usize, // number of times item appears in transactions
}

#[derive(Default, Debug)]
struct TreeNode {
    label: usize,
    first_child: Option<Rc<RefCell<TreeNode>>>,
    next: Option<Rc<RefCell<TreeNode>>>,
    tid_set: RoaringBitmap,
    count: usize,
}

impl NEclatClosed {
    /// Run the algorithm.
    pub fn process(mut self, transactions: &Vec<Vec<u32>>, min_support: f32) -> Vec<ItemSet> {
        // Read Dataset
        self.get_data(transactions, min_support);

        self.itemset_x = vec![0; self.n_items];

        // Build tree
        self.build_tree(transactions);

        // Initialize tree
        self.init_tree();

        // Find closed itemsets
        let mut cur_node = self.nl_root.first_child.clone();
        self.nl_root.first_child = None;
        let mut next;
        while let Some(ref child) = cur_node {
            self.traverse(child, 1);
            next = child.borrow().next.clone();
            child.borrow_mut().next = None;
            cur_node = next;
        }

        self.closed_itemsets
    }

    /// Read dataset to find the frequent items.
    fn get_data(&mut self, transactions: &Vec<Vec<u32>>, min_support: f32) {
        self.n_transactions = 0;

        // (1) Scan the database and count the count of each item.
        // The count of items is stored in map where
        // key = item value => count count
        let mut item_count: HashMap<usize, usize> = HashMap::new();
        for transaction in transactions {
            self.n_transactions += 1;
            for id in transaction {
                let count = item_count.entry(*id as usize).or_insert(0);
                *count += 1;
            }
        }

        self.min_support = (min_support * (self.n_transactions as f32)).ceil() as usize;

        for (id, count) in item_count {
            if count >= self.min_support {
                self.items.push(Item {
                    index: id,
                    num: count,
                });
            }
        }

        self.n_items = self.items.len();

        // Sort the items by decreasing order of frequency.
        self.items.sort_by(|a, b| b.num.cmp(&a.num));
    }

    /// Build the tree.
    fn build_tree(&mut self, transactions: &[Vec<u32>]) {
        for (tid, transaction) in transactions.iter().enumerate() {
            for id in transaction {
                // Add each item from the transaction except infrequent item
                for (i, item) in self.items.iter().enumerate() {
                    // If the item appears in the list of frequent items, we add it
                    if item.index == *id as usize {
                        // Get the current tidset of that item, if it doesn't exist, create it
                        let tids = self.item_tids.entry(i).or_insert_with(RoaringBitmap::new);
                        // We add the current transaction id to the tidset of the item
                        tids.insert(tid as u32);
                        break;
                    }
                }
            }
        }
    }

    /// Initialize the tree.
    fn init_tree(&mut self) {
        self.nl_root.label = self.n_items;

        let mut last_child = None;

        for (t, _item) in self.items.iter().enumerate().rev() {
            let mut node = TreeNode::default();
            node.label = t;
            node.first_child = None;
            node.next = None;
            node.tid_set = self.item_tids[&node.label].clone();
            node.count = node.tid_set.len() as usize;

            let node_rc = Rc::new(RefCell::new(node));

            if self.nl_root.first_child.is_none() {
                self.nl_root.first_child = Some(node_rc.clone());
                last_child = Some(node_rc);
            } else if let Some(ref child) = last_child {
                child.borrow_mut().next = Some(node_rc.clone());
                last_child = Some(node_rc);
            }
        }
    }

    /// Recursively constructing_frequent_itemset_tree the tree to find frequent itemsets
    fn traverse(&mut self, curr: &Rc<RefCell<TreeNode>>, level: usize) {
        let mut sibling = curr.borrow().next.clone();
        let mut last_child = None;
        let mut same_count = 0;

        self.itemset_x[self.itemset_x_len] = curr.borrow().label;
        self.itemset_x_len += 1;

        while let Some(s) = sibling {
            let mut child = TreeNode::default();
            if level == 1 && !s.borrow().tid_set.is_empty() {
                child.tid_set = &curr.borrow().tid_set - &s.borrow().tid_set;
            } else if !curr.borrow().tid_set.is_empty() {
                child.tid_set = &s.borrow().tid_set - &curr.borrow().tid_set;
            }

            child.count = curr.borrow().count - child.tid_set.len() as usize;
            if child.count >= self.min_support {
                if curr.borrow().count == child.count {
                    self.itemset_x[self.itemset_x_len] = s.borrow().label;
                    self.itemset_x_len += 1;
                    same_count += 1;
                } else {
                    child.label = s.borrow().label;
                    child.first_child = None;
                    child.next = None;

                    let child_rc = Rc::new(RefCell::new(child));

                    if curr.borrow().first_child.is_none() {
                        curr.borrow_mut().first_child = Some(child_rc.clone());
                        last_child = Some(child_rc);
                    } else if let Some(child) = last_child {
                        child.borrow_mut().next = Some(child_rc.clone());
                        last_child = Some(child_rc);
                    }
                }
            }
            sibling = s.borrow().next.clone();
        }

        let itemset_bitvec = BitVector::new(&self.itemset_x, self.itemset_x_len);
        if self
            .cp_storage
            .insert_if_close(itemset_bitvec, curr.borrow().count)
        {
            // create a stringbuffer
            let mut indices = Vec::with_capacity(self.itemset_x_len);
            // append items from the itemset to the stringbuffer
            for i in 0..self.itemset_x_len {
                indices.push(self.items[self.itemset_x[i]].index);
            }

            self.closed_itemsets.push(ItemSet {
                indices,
                support: curr.borrow().count,
            });
        }

        let mut child_opt = curr.borrow().first_child.clone();
        let mut next;
        curr.borrow_mut().first_child = None;
        while let Some(ref child) = child_opt {
            self.traverse(child, level + 1);
            next = child.borrow().next.clone();
            child.borrow_mut().next = None;
            child_opt = next;
        }

        self.itemset_x_len -= 1 + same_count;
    }
}

#[cfg(test)]
mod test {
    use std::{
        env,
        fs::File,
        io::{BufRead, BufReader},
    };

    use super::*;

    #[test]
    fn test_neclatclosed() {
        let neclat = NEclatClosed::default();

        let mut transactions = Vec::new();

        // get the root directory of the project
        let project_root_path = env::var("CARGO_MANIFEST_DIR").unwrap();

        let dataset_path = format!("{}/tests/data/chess.dat", project_root_path);

        // open test dataset file
        let dataset = File::open(dataset_path).unwrap();
        let reader = BufReader::new(dataset);

        for line in reader.lines() {
            if let Ok(line) = line {
                let line = line.split_whitespace().collect::<Vec<&str>>();
                transactions.push(line.iter().map(|x| x.parse::<u32>().unwrap()).collect());
            }
        }

        // chess.data.out is generated by the Java implementation of NEclatClosed,
        // which the min_support is 0.9. So we set the min_support to 0.9 here. Just
        // for testing.
        let result = neclat.process(&transactions, 0.9);

        let mut formated_result = result
            .iter()
            .map(|x| {
                let mut line = x.indices.clone();
                line.insert(0, x.support);
                format!(
                    "{}",
                    line.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            })
            .collect::<Vec<String>>();

        // sort the result line
        formated_result.sort();

        // read the expected result from the file
        let expected_result =
            File::open(format!("{}/tests/data/chess.dat.out", project_root_path)).unwrap();
        let reader = BufReader::new(expected_result);
        let mut expected_result_vec = Vec::new();

        for line in reader.lines() {
            if let Ok(line) = line {
                expected_result_vec.push(line);
            }
        }
        // sort the expected result line
        expected_result_vec.sort();

        assert_eq!(formated_result, expected_result_vec);
    }
}
