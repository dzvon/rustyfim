use crate::{
	DataSet,
	ItemSet,
	Support,
	closed::{
		InitialSets,
		is_dup
	}
};


/// Sequential implementation of the DCI-Closed algorithm. This is a straightforward
/// implementation of the original algorithm from the paper. It uses only one CPU core.
///
/// The returned collection will **always** have at least one element, with maximum support:
/// the set of all elements which occur in all transactions.
pub fn closed<D>(dataset: &D, min_sup: usize) -> Box<[(D::ItemSet, Support)]>
where
	D: DataSet,
	for<'b> &'b D::ItemSet: IntoIterator<Item = usize>
{
	let InitialSets { closed, pre, post } = InitialSets::new(dataset, min_sup);

	let closed_set = closed.clone();
	let transactions_count = dataset.transactions_count();

	let mut closed_itemsets = vec![(closed_set, transactions_count)];

	_closed(
		dataset,
		min_sup,
		&closed,
		pre,
		&post,
		&mut closed_itemsets
	);

	closed_itemsets.into_boxed_slice()
}


fn _closed<'a, D>(
	dataset: &'a D,
	min_sup: usize,
	closed_set: &D::ItemSet,
	mut pre_set: D::ItemSet,
	post_set: &D::ItemSet,
	out: &mut Vec<(D::ItemSet, Support)>,
)
where
	D: DataSet,
	for<'b> &'b D::ItemSet: IntoIterator<Item = usize>
{
	for i in post_set.into_iter() {
		let mut new_gen = closed_set.clone();
		new_gen.add(i);

		let new_gen_cover = dataset.cover(&new_gen);

		if (dataset.support(&new_gen) >= min_sup) && (!is_dup(dataset, &new_gen_cover, &pre_set)) {
			let mut closed_set_new = new_gen.clone();
			let mut post_set_new = D::ItemSet::empty();

			for j in post_set.into_iter().skip_while(|&j| i >= j) {
				if dataset.supports(j, new_gen_cover) {
					closed_set_new.add(j);
				}
				else {
					post_set_new.add(j)
				}
			}

			out.push(
				(
					closed_set_new.clone(),
					dataset.support(&closed_set_new)
				)
			);

			let pre_set_new = pre_set.clone();

			_closed(
				dataset,
				min_sup,
				&closed_set_new,
				pre_set_new,
				&post_set_new,
				out,
			);

			pre_set.add(i);
		}
	}
}
