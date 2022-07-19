use std::marker::PhantomData;

use bitmatrix::BitMatrix;
use bitvec::{boxed::BitBox, vec::BitVec};

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
