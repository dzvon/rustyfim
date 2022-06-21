use super::{
	DataSet,
	ItemSet,
};


/// The initial sets for the algorithm.
pub struct InitialSets<ItemSet> {
	/// Itemset with all items that occur in all transactions.
	pub closed: ItemSet,
	/// Always empty.
	pub pre: ItemSet,
	/// Itemset with all frequent items, except those in closed.
	pub post: ItemSet,
}


impl<IS: ItemSet> InitialSets<IS> {
	pub fn new<D>(dataset: &D, min_sup: usize) -> Self
	where
		D: DataSet<ItemSet = IS>,
	{
		let mut closed = D::ItemSet::empty();
		let pre = D::ItemSet::empty();
		let mut post = D::ItemSet::empty();

		let transactions_count = dataset.transactions_count();

		for i in 0 .. dataset.items_count() {
			let support = dataset.item_support(i);

			if support == transactions_count {
				closed.add(i);
			}
			else if support >= min_sup {
				post.add(i);
			}
		}

		Self { closed, pre, post }
	}
}


pub fn is_dup<D>(dataset: &D, new_gen_cover: &D::Cover, pre_set: &D::ItemSet) -> bool
where
	D: DataSet,
	for<'a> &'a D::ItemSet: IntoIterator<Item = usize>
{
	for item in pre_set.into_iter() {
		if dataset.supports(item, &new_gen_cover) {
			return true;
		}
	}

	false
}
