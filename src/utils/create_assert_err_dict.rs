use std::collections::HashMap;

use atomic_bomb_engine::models::assert_error_stats::AssertErrKey;
use pyo3::types::{PyDict, PyDictMethods, PyList};
use pyo3::{Py, PyResult, Python};

pub fn create_assert_error_dict(
    py: Python<'_>,
    assert_errors: &HashMap<AssertErrKey, u32>,
) -> PyResult<Py<PyList>> {
    if assert_errors.is_empty() {
        return Ok(PyList::empty(py).unbind());
    }

    let mut result_errors: Vec<Py<PyDict>> = Vec::with_capacity(assert_errors.len());
    for (assert_error_key, count) in assert_errors {
        let assert_error_dict = PyDict::new(py);
        assert_error_dict.set_item("name", &assert_error_key.name)?;
        assert_error_dict.set_item("message", &assert_error_key.msg)?;
        assert_error_dict.set_item("url", &assert_error_key.url)?;
        assert_error_dict.set_item("host", &assert_error_key.host)?;
        assert_error_dict.set_item("path", &assert_error_key.path)?;
        assert_error_dict.set_item("count", count)?;
        result_errors.push(assert_error_dict.unbind());
    }

    Ok(PyList::new(py, result_errors)?.unbind())
}
