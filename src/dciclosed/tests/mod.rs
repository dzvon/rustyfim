mod datasets;

use std::{
	collections::HashSet,
	fmt::Debug,
	hash::Hash,
};

use crate::{DataSet, Support};
use self::datasets::TestDataSet;


/// Test the sequential implementation with the given test dataset.
fn test_sequential<D>(dataset: &TestDataSet<D>)
where
	D: DataSet,
	D::ItemSet: Debug + Eq + Hash,
	for<'b> &'b D::ItemSet: IntoIterator<Item = usize>,
{
	let result: HashSet<(D::ItemSet, Support)> = crate::sequential
		::closed(&dataset.dataset, dataset.min_sup)
		.into_vec() // Boxed slice has no owned iterator.
		.into_iter()
		.collect();

	assert_eq!(result, dataset.result)
}

/// Test the parallel implementation with the given test dataset.
fn test_parallel<D>(dataset: &TestDataSet<D>)
where
	D: DataSet + Sync,
	D::ItemSet: Debug + Eq + Hash + Send + Sync,
	for<'b> &'b D::ItemSet: IntoIterator<Item = usize>,
{
	let result: HashSet<(D::ItemSet, Support)> = crate::parallel
		::closed(&dataset.dataset, dataset.min_sup)
		.into_vec()
		.into_iter()
		.collect();

	assert_eq!(result, dataset.result)
}


#[test]
fn test_toy() {
	let dataset = &datasets::TOY;

	test_sequential(dataset);
	test_parallel(dataset);
}
