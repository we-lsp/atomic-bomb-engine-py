use pyo3::types::{PyAny, PyDict, PyDictMethods};
use pyo3::{pyfunction, Py, PyResult, Python};

#[pyfunction]
#[pyo3(signature=(
key,
jsonpath,
))]
pub(crate) fn jsonpath_extract_option(
    py: Python,
    key: String,
    jsonpath: String,
) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(py);
    dict.set_item("key", key)?;
    dict.set_item("jsonpath", jsonpath)?;
    Ok(dict.into_any().unbind())
}
