use std::collections::HashSet;
use bitmatrix::BitMatrix;
use crate::matrix::{Matrix, ItemSet8};

use lazy_static::lazy_static;

use crate::{
	DataSet,
	Support
};


/// Generic type for a test dataset.
pub struct TestDataSet<D: DataSet> {
	pub dataset: D,
	pub result: HashSet<(D::ItemSet, Support)>,
	pub min_sup: usize,
}

lazy_static! {
	pub static ref TOY: TestDataSet<Matrix::<ItemSet8>> = TestDataSet {
		dataset: {
			let mut matrix = BitMatrix::new(5, 6);
			matrix.set((0, 0), true);
			matrix.set((0, 3), true);
			matrix.set((0, 5), true);
			matrix.set((1, 1), true);
			matrix.set((1, 4), true);
			matrix.set((2, 0), true);
			matrix.set((2, 1), true);
			matrix.set((2, 2), true);
			matrix.set((2, 4), true);
			matrix.set((2, 5), true);
			matrix.set((3, 0), true);
			matrix.set((3, 3), true);
			let dataset = Matrix::from(matrix);

			dataset
		},

		result: [
			([0, 2].iter().copied().into(), 2),
			([0, 3].iter().copied().into(), 2),
			([1, 2].iter().copied().into(), 2),
			([0].iter().copied().into(), 3),
			([2].iter().copied().into(), 5),
			([].iter().copied().into(), 6),
		]
			.iter()
			.cloned()
			.collect(),

		min_sup: 2,
	};
}
