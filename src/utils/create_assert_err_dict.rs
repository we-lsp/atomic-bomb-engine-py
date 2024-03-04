use std::collections::HashMap;
use pyo3::{PyResult, Python};
use pyo3::types::{PyDict, PyList};
pub fn create_assert_error_dict<'py>(py: Python<'py>, assert_errors: &HashMap<(String, String), u32>) -> PyResult<&'py PyList> {
    if assert_errors.is_empty() {
        return Ok(PyList::empty(py));
    }

    let mut result_errors = Vec::new();
    for ((url, message), count) in assert_errors {
        let assert_error_dict = PyDict::new(py);

        assert_error_dict.set_item("url", url)?;
        assert_error_dict.set_item("message", message)?;
        assert_error_dict.set_item("count", count)?;

        result_errors.push(assert_error_dict)
    }
    Ok(PyList::new(py, result_errors))
}
