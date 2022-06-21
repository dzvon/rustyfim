use std::marker::PhantomData;
use std::fmt;
use pyo3::prelude::*;

use bitvec::{boxed::BitBox, vec::BitVec};
use bitmatrix::BitMatrix;

use super::{DataSet, ItemSet};

/// A DataSet implemented as a BitMatrix, from the `bitmatrix` crate. This implementation
/// provides optimal performance for the operations required by the DCI-Closed algorithm.
///
/// Use the `From` trait to instantiate this type.
///
/// This type is generic over the ItemSet type, which can be specialized for the number of
/// items in the dataset. Once again, it is higly recommend that such itemset type is
/// implemented using a bitset. The itemset must also provide an IntoIterator
/// implementation:
///
/// ```rust,ignore
/// for<'a> &'a I: IntoIterator<Item = usize>
/// ```
///
/// This implementation **must** yield the items in lexicographic order.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Matrix<I>(pub BitMatrix, PhantomData<I>);

impl<I> DataSet for Matrix<I>
where
	I: ItemSet,
	for<'a> &'a I: IntoIterator<Item = usize>,
{
	type ItemSet = I;
	type Cover = BitBox;

	fn items_count(&self) -> usize {
		self.0.height()
	}

	fn transactions_count(&self) -> usize {
		self.0.width()
	}

	fn item_support(&self, item: usize) -> usize {
		self.0[item].count_ones()
	}

	fn support(&self, itemset: &Self::ItemSet) -> usize {
		self.cover(itemset).count_ones()
	}

	fn supports(&self, item: usize, cover: &Self::Cover) -> bool {
		let item_iter = self.0[item].iter();
		let cover_iter = cover.iter();

		item_iter.zip(cover_iter).all(|(&a, &b)| (!b) || a)
	}

	fn cover(&self, itemset: &Self::ItemSet) -> Self::Cover {
		let length = self.transactions_count();

		let mut cover = {
			let mut vec = BitVec::with_capacity(length);
			vec.resize(length, true);
			vec.into_boxed_bitslice()
		};

		for item in itemset.into_iter() {
			cover &= self.0[item].iter().copied();
		}

		cover
	}
}

impl<I> From<BitMatrix> for Matrix<I> {
	fn from(matrix: BitMatrix) -> Self {
		Matrix(matrix, PhantomData)
	}
}

impl<I> Into<BitMatrix> for Matrix<I> {
	fn into(self) -> BitMatrix {
		self.0
	}
}

impl<'a, I> Into<&'a BitMatrix> for &'a Matrix<I> {
	fn into(self) -> &'a BitMatrix {
		&self.0
	}
}

/// A itemset capable of storing up to 8 items. For test purposes only.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemSet8(u8);

pub struct ItemSet8Iter {
	ix: usize,
	data: u8,
}

impl Iterator for ItemSet8Iter {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		if self.ix == 8 {
			return None;
		}

		let bit = self.data & (1 << self.ix);
		self.ix += 1;

		if bit == 0 {
			self.next()
		} else {
			Some(self.ix - 1)
		}
	}
}

impl<'a> IntoIterator for &'a ItemSet8 {
	type Item = usize;
	type IntoIter = ItemSet8Iter;

	fn into_iter(self) -> Self::IntoIter {
		ItemSet8Iter {
			ix: 0,
			data: self.0,
		}
	}
}

impl ItemSet for ItemSet8 {
	fn empty() -> Self {
		Self(0)
	}
	fn add(&mut self, item: usize) {
		assert!(item < 8);
		self.0 |= 1 << item;
	}
}

impl fmt::Debug for ItemSet8 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut iter = self.into_iter();

		f.write_str("{")?;

		if let Some(i) = iter.next() {
			write!(f, "{}", i)?;
		}

		for i in iter {
			write!(f, ", {}", i)?;
		}

		f.write_str("}")?;

		Ok(())
	}
}

impl<I> From<I> for ItemSet8
where
	I: Iterator<Item = usize>,
{
	fn from(iter: I) -> Self {
		let mut itemset = Self::empty();

		for i in iter {
			itemset.add(i);
		}

		itemset
	}
}

impl IntoPy<PyObject> for ItemSet8 {
    fn into_py(self, py: Python) -> PyObject {
		let mut vec = Vec::new();
		for i in self.into_iter() {
			vec.push(i)
		}
		vec.into_py(py)
    }
}
