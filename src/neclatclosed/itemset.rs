use pyo3::{IntoPy, PyObject, Python};

#[derive(Debug, PartialEq, Eq)]
pub struct ItemSet {
    pub indices: Vec<usize>,
    pub support: usize,
}

impl IntoPy<PyObject> for ItemSet {
    fn into_py(self, py: Python) -> PyObject {
        // return indices and support as a tuple
        (self.indices, self.support).into_py(py)
    }
}
