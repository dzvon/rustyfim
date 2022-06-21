use super::{
	closed::{is_dup, InitialSets},
	DataSet, ItemSet, Support,
};

/// Parallel implementation of the DCI-Closed algorithm. This implementation is a simple
/// adaptation of the original algorithm from the paper, using Rayon. It should use all the
/// CPU cores.
///
/// The returned collection will **always** have at least one element, with maximum support:
/// the set of all elements which occur in all transactions.
///
/// # Panics
/// This should never really panic, but it might happen if something really bad happens with
/// Rayon or the spawned tasks.
pub fn closed<D>(dataset: &D, min_sup: usize) -> Box<[(D::ItemSet, Support)]>
where
	D: DataSet + Sync,
	D::ItemSet: Send + Sync,
	for<'a> &'a D::ItemSet: IntoIterator<Item = usize>,
{
	// The bound for the channel. This value was chosen rather arbitrarily.
	const CHANNEL_BOUND: usize = 8192;

	let InitialSets { closed, pre, post } = InitialSets::new(dataset, min_sup);

	let closed_set = closed.clone();
	let transactions_count = dataset.transactions_count();

	let (tx, rx) = std::sync::mpsc::sync_channel(CHANNEL_BOUND);

	let collector = std::thread::spawn(move || -> Box<[(D::ItemSet, Support)]> {
		// Should never panic.
		let mut vec = vec![(closed_set, transactions_count)];
		vec.extend(rx.iter());
		vec.into_boxed_slice()
	});

	rayon::scope(|scope| _closed(dataset, min_sup, &closed, pre, &post, &tx, scope));

	drop(tx);

	collector.join().expect("Failed to join collector thread") // The collector should never panic.
}

fn _closed<'a, D>(
	dataset: &'a D,
	min_sup: usize,
	closed_set: &D::ItemSet,
	mut pre_set: D::ItemSet,
	post_set: &D::ItemSet,
	out: &'a std::sync::mpsc::SyncSender<(D::ItemSet, Support)>,
	scope: &rayon::Scope<'a>,
) where
	D: DataSet + Sync,
	D::ItemSet: Send + Sync,
	for<'b> &'b D::ItemSet: IntoIterator<Item = usize>,
{
	for i in post_set.into_iter() {
		let mut new_gen = closed_set.clone();
		new_gen.add(i);

		let new_gen_cover = dataset.cover(&new_gen);

		if dataset.support(&new_gen) >= min_sup {
			if !is_dup(dataset, &new_gen_cover, &pre_set) {
				let mut closed_set_new = new_gen.clone();
				let mut post_set_new = D::ItemSet::empty();

				for j in post_set.into_iter().skip_while(|&j| i >= j) {
					if dataset.supports(j, &new_gen_cover) {
						closed_set_new.add(j);
					} else {
						post_set_new.add(j)
					}
				}

				out.send((closed_set_new.clone(), dataset.support(&closed_set_new)))
					// The receiver should never be closed before all tasks finish.
					// If that happens for some extraordinary reason, propagate the panic to rayon.
					.expect("Failed to send result");

				let pre_set_new = pre_set.clone();

				scope.spawn(move |scope| {
					_closed(
						dataset,
						min_sup,
						&closed_set_new,
						pre_set_new,
						&post_set_new,
						out,
						scope,
					)
				});

				pre_set.add(i);
			}
		}
	}
}
