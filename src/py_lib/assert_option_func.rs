use pyo3::{pyfunction, PyObject, PyResult, Python, ToPyObject};
use pyo3::types::PyDict;

#[pyfunction]
pub(crate) fn assert_option(py: Python, jsonpath: String, reference_object: PyObject) -> PyResult<PyObject>{
    let dict = PyDict::new(py);
    dict.set_item("jsonpath", jsonpath)?;
    dict.set_item("reference_object", reference_object)?;
    Ok(dict.to_object(py))
}
