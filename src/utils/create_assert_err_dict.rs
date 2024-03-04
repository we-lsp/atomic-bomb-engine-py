use std::collections::HashMap;
use pyo3::{PyResult, Python};
use pyo3::types::{PyDict, PyList};
pub fn create_assert_error_dict<'py>(py: Python<'py>, assert_errors: &HashMap<(String, String), u32>) -> PyResult<&'py PyDict> {
    if assert_errors.is_empty() {
        return Ok(PyDict::new(py));
    }

    let assert_error_dict = PyDict::new(py);
    for ((url, message), count) in assert_errors {
        assert_error_dict.set_item("url", url)?;
        assert_error_dict.set_item("message", message)?;
        assert_error_dict.set_item("count", count)?;
    }
    Ok(assert_error_dict)
}
