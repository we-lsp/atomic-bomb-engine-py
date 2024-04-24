use pyo3::types::PyDict;
use pyo3::{pyfunction, PyObject, PyResult, Python, ToPyObject};

#[pyfunction]
pub(crate) fn jsonpath_extract_option(
    py: Python,
    key: String,
    jsonpath: String,
) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    dict.set_item("key", key)?;
    dict.set_item("jsonpath", jsonpath)?;
    Ok(dict.to_object(py))
}
