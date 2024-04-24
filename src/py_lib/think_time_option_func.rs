use pyo3::types::PyDict;
use pyo3::{pyfunction, PyObject, PyResult, Python, ToPyObject};

#[pyfunction]
pub(crate) fn think_time_option(
    py: Python,
    min_millis: u64,
    max_millis: u64,
) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    dict.set_item("min_millis", min_millis)?;
    dict.set_item("max_millis", max_millis)?;
    Ok(dict.to_object(py))
}
