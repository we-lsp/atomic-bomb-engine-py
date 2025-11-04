use pyo3::types::{PyDict, PyDictMethods};
use pyo3::{pyfunction, Py, PyResult, Python};
use pyo3::types::PyAny;

#[pyfunction]
#[pyo3(signature=(
jsonpath,
reference_object,
))]
pub(crate) fn assert_option(
    py: Python,
    jsonpath: String,
    reference_object: Py<PyAny>,
) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(py);
    dict.set_item("jsonpath", jsonpath)?;
    dict.set_item("reference_object", reference_object)?;
    Ok(dict.into_any().unbind())
}
