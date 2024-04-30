use atomic_bomb_engine::models::assert_error_stats::AssertErrKey;
use pyo3::types::{PyDict, PyList};
use pyo3::{PyResult, Python};
use std::collections::HashMap;

pub fn create_assert_error_dict<'py>(
    py: Python<'py>,
    assert_errors: &HashMap<AssertErrKey, u32>,
) -> PyResult<&'py PyList> {
    if assert_errors.is_empty() {
        return Ok(PyList::empty(py));
    }

    let mut result_errors = Vec::new();
    for (assert_error_key, count) in assert_errors {
        let assert_error_dict = PyDict::new(py);
        let assert_err_key_clone = assert_error_key.clone();

        assert_error_dict.set_item("name", assert_err_key_clone.name)?;
        assert_error_dict.set_item("message", assert_err_key_clone.msg)?;
        assert_error_dict.set_item("url", assert_err_key_clone.url)?;
        assert_error_dict.set_item("host", assert_err_key_clone.host)?;
        assert_error_dict.set_item("path", assert_err_key_clone.path)?;
        assert_error_dict.set_item("count", count)?;

        result_errors.push(assert_error_dict)
    }
    Ok(PyList::new(py, result_errors))
}
