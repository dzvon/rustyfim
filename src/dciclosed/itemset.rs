use pyo3::prelude::*;

use bitvec::{array::BitArray, order::Lsb0};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemSet(
    BitArray<
        Lsb0,
        // TODO(tony): pass one hot length dynamically as feature grows
        [usize; bitvec::mem::elts::<usize>(500 as usize)],
    >,
);

impl<'a> IntoIterator for &'a ItemSet {
    type Item = usize;

    type IntoIter = std::iter::FilterMap<
        std::iter::Enumerate<bitvec::slice::Iter<'a, Lsb0, usize>>,
        fn((usize, &bool)) -> Option<usize>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(ix, item)| if *item { Some(ix) } else { None })
    }
}

impl super::ItemSet for ItemSet {
    fn empty() -> Self {
        Self(BitArray::zeroed())
    }

    fn add(&mut self, item: usize) {
        self.0.set(item, true);
    }
}

impl IntoPy<PyObject> for ItemSet {
    fn into_py(self, py: Python) -> PyObject {
        let mut vec = Vec::new();
        for i in self.into_iter() {
            vec.push(i)
        }
        vec.into_py(py)
    }
}
