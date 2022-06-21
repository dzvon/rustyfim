pub mod matrix;
pub mod parallel;
pub mod closed;

/// The support type, i.e. how many transactions contain a given item or itemset.
pub type Support = usize;

/// A trait for a DataSet. The algorithm can operate over any type that implements this
/// trait. It is highly recommended to implement this datastruct as a bit matrix, and its
/// associated types as bitsets, in order to obtain optimal performance.
pub trait DataSet {
	/// The type of a itemset for this dataset.
	type ItemSet: ItemSet;

	/// The type for a cover for this dataset. A cover is the set of transactions that
	/// supports an itemset. To obtain optimal performance, this should be implemented
	/// as a bitset.
	type Cover;

	/// How many transactions in the dataset.
	fn transactions_count(&self) -> usize;
	/// How many items in the dataset.
	fn items_count(&self) -> usize;

	/// Calculate the support of a given item.
	fn item_support(&self, item: usize) -> Support;
	/// Calculate the support of a given itemset.
	fn support(&self, itemset: &Self::ItemSet) -> Support;

	/// Calculate the cover of a given itemset.
	fn cover(&self, itemset: &Self::ItemSet) -> Self::Cover;
	/// Check if an item supports all transactions in the given cover.
	fn supports(&self, item: usize, cover: &Self::Cover) -> bool;
}


/// A trait for an ItemSet. ItemSets must be owned, cloneable and iterable.
/// Once again, a bitset implementation is highly recommended.
///
/// An ItemSet must also provide an IntoIterator implementation:
///
/// ```rust,ignore
/// for<'a> &'a I: IntoIterator<Item = usize>
/// ```
///
/// This implementation **must yield the items in lexicographic order**.
pub trait ItemSet: Clone + 'static {
	/// Create an empty ItemSet.
	fn empty() -> Self;
	/// Add an item to the ItemSet.
	fn add(&mut self, item: usize);
}
