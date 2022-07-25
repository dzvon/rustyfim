use neclatclosed::ItemSet as NeclatClosedItemSet;
pub use neclatclosed::NEclatClosed;

use pyo3::prelude::{pyfunction, pymodule, wrap_pyfunction, PyModule, PyResult, Python};

mod neclatclosed;

#[pyfunction]
fn neclat(min_support: f32, transactions: Vec<Vec<u32>>) -> PyResult<Vec<NeclatClosedItemSet>> {
    let neclat = NEclatClosed::default();

    let result = neclat.process(&transactions, min_support);

    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustyfim(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(neclat, m)?)?;
    Ok(())
}
