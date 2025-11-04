use pyo3::types::PyDict;
use pyo3::{pyfunction, PyObject, PyResult, Python};
use pyo3::prelude::PyDictMethods;

#[pyfunction]
#[pyo3(signature=(
jsonpath,
reference_object,
))]
pub(crate) fn assert_option(
    py: Python,
    jsonpath: String,
    reference_object: PyObject,
) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    dict.set_item("jsonpath", jsonpath)?;
    dict.set_item("reference_object", reference_object)?;
    Ok(dict.into_any().unbind())
}
