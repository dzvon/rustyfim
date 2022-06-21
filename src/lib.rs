use pyo3::prelude::{pyfunction, pymodule, wrap_pyfunction, PyModule, PyResult, Python};
use std::error::Error;
use std::time::{Duration, Instant};

mod fp;
use fp::{fptree::{fp_growth, FPTree, ItemSet}, item::Item, item_counter::ItemCounter};

mod dciclosed;
use dciclosed::matrix::{Matrix, ItemSet8};
use bitmatrix::BitMatrix;


fn duration_as_ms(duration: &Duration) -> u64 {
    (duration.as_secs() * 1_000 as u64) + (duration.subsec_nanos() / 1_000_000) as u64
}

fn count_item_frequencies(
    transactions: &Vec<Vec<u32>>,
) -> Result<(ItemCounter, u32), Box<dyn Error>> {
    let mut item_count: ItemCounter = ItemCounter::new();
    let mut num_transactions = 0;
    for transaction in transactions {
        num_transactions += 1;
        for id in transaction.iter() {
            item_count.add(&Item::with_id(*id), 1);
        }
    }
    Ok((item_count, num_transactions))
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn fpg(min_support: f32, transactions: Vec<Vec<u32>>) -> PyResult<Vec<ItemSet>> {
    let start = Instant::now();
    let timer = Instant::now();
    let (item_count, num_transactions) = count_item_frequencies(&transactions).unwrap();
    println!(
        "First pass took {} ms, num_transactions={}.",
        duration_as_ms(&timer.elapsed()),
        num_transactions
    );

    let mut fptree = FPTree::new();
    let min_count = 1.max((min_support * (num_transactions as f32)).ceil() as u32);
    for transaction in transactions {
        // Strip out infrequent items from the transaction. This can
        // drastically reduce the tree size, and speed up loading the
        // initial tree.
        let mut filtered_transaction = transaction
            .into_iter()
            .map(|id| Item::with_id(id))
            .filter(|item| item_count.get(&item) > min_count)
            .collect::<Vec<Item>>();
        item_count.sort_descending(&mut filtered_transaction);
        fptree.insert(&filtered_transaction, 1);
    }

    let patterns: Vec<ItemSet> = fp_growth(&fptree, min_count, &vec![], num_transactions as u32);

    println!("Total runtime: {} ms", duration_as_ms(&start.elapsed()));

    Ok(patterns)
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn dci(min_support: f32, transactions: Vec<Vec<u32>>, n_features: usize) -> PyResult<Vec<(ItemSet8, usize)>> {
    let start = Instant::now();

	let n_transactions: usize = transactions.len();
	let mut matrix: BitMatrix = BitMatrix::new(n_transactions, n_features);
	for (i, transaction) in transactions.iter().enumerate() {
        for id in transaction.iter() {
            matrix.set((i, *id as usize), true);
        }
    }
	
	let min_sup: usize = (min_support * (n_transactions as f32)) as usize;
	let result: Vec<(ItemSet8, usize)> = dciclosed::parallel::closed(&Matrix::from(matrix), min_sup).into_vec();

    println!("Total runtime: {} ms", duration_as_ms(&start.elapsed()));

    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustyfim(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fpg, m)?)?;
    m.add_function(wrap_pyfunction!(dci, m)?)?;
    Ok(())
}
