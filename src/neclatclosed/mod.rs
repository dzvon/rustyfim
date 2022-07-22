//! NElcatClosed algorithm implementation.

mod bitvector;
mod cpstorage;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

// use bitvec1::{bitvec, vec::BitVec};
use roaring::RoaringBitmap;

use bitvector::BitVector;

use self::cpstorage::CPStorage;

#[derive(Default)]
struct NEclatClosed {
    output_cnt: usize,   // number of itemsets found
    num_of_items: usize, // number of items
    min_support: usize,  // minimum support threshold
    items: Vec<Item>,    // list of items sorted by count

    num_of_transactions: usize, // number of transactions

    itemset_x: Vec<usize>, // the current itemset
    itemset_x_len: usize,  // the length of the current itemset

    // A map containing the tidset (i.e. cover) of each item represented as a bitvector.
    item_tids: HashMap<usize, RoaringBitmap>,

    nl_root: TreeNode, // root of the tree

    cp_storage: CPStorage, // storage for closed patterns

    closed_itemsets: Vec<ItemSet>, // list of closed itemsets
}

#[derive(PartialEq, Eq, Debug)]
struct Item {
    index: usize,
    num: usize, // number of times item appears in transactions
}

#[derive(Debug)]
struct ItemSet {
    indices: Vec<usize>,
    support: usize,
}

impl ItemSet {
    pub fn new(indices: Vec<usize>, support: usize) -> ItemSet {
        ItemSet { indices, support }
    }
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
    pub fn process(mut self, transactions: &Vec<Vec<usize>>, min_support: f32) -> Vec<ItemSet> {
        // Read Dataset
        self.get_data(transactions, min_support);

        self.itemset_x = vec![0; self.num_of_items];

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
    fn get_data(&mut self, transactions: &Vec<Vec<usize>>, min_support: f32) {
        self.num_of_transactions = 0;

        // (1) Scan the database and count the count of each item.
        // The count of items is stored in map where
        // key = item value => count count
        let mut item_count: HashMap<usize, usize> = HashMap::new();
        for transaction in transactions {
            self.num_of_transactions += 1;
            for id in transaction {
                let count = item_count.entry(*id).or_insert(0);
                *count += 1;
            }
        }

        self.min_support = (min_support * (self.num_of_transactions as f32)).ceil() as usize;

        for (id, count) in item_count {
            if count >= self.min_support {
                self.items.push(Item {
                    index: id,
                    num: count,
                });
            }
        }

        self.num_of_items = self.items.len();

        // Sort the items by decreasing order of frequency.
        self.items.sort_by(|a, b| b.num.cmp(&a.num));
    }

    /// Build the tree.
    fn build_tree(&mut self, transactions: &[Vec<usize>]) {
        for (tid, transaction) in transactions.iter().enumerate() {
            for id in transaction {
                // Add each item from the transaction except infrequent item
                for (i, item) in self.items.iter().enumerate() {
                    // If the item appears in the list of frequent items, we add it
                    if item.index == *id {
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
        self.nl_root.label = self.num_of_items;

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
        fs::File,
        io::{BufRead, BufReader},
    };

    use super::*;

    #[test]
    fn test_neclatclosed() {
        let neclat = NEclatClosed::default();

        let mut transactions = Vec::new();

        // open dataset file
        let dataset = File::open("/Users/pvon/Downloads/connect.dat").unwrap();

        let reader = BufReader::new(dataset);

        for line in reader.lines() {
            if let Ok(line) = line {
                let line = line.split_whitespace().collect::<Vec<&str>>();
                transactions.push(line.iter().map(|x| x.parse::<usize>().unwrap()).collect());
            }
        }

        let start = std::time::Instant::now();
        let result = neclat.process(&transactions, 0.3);
        println!("{:?}", start.elapsed());

        // println!("{result:?}");
        println!("{}", result.len());
    }
}
