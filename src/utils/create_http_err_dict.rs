use std::collections::HashMap;

use atomic_bomb_engine::models::http_error_stats::HttpErrKey;
use pyo3::prelude::PyDictMethods;
use pyo3::types::{PyDict, PyList};
use pyo3::{Py, PyResult, Python};

pub fn create_http_error_dict(
    py: Python<'_>,
    http_errors: &HashMap<HttpErrKey, u32>,
) -> PyResult<Py<PyList>> {
    if http_errors.is_empty() {
        return Ok(PyList::empty(py).unbind());
    }

    let mut http_errors_list: Vec<Py<PyDict>> = Vec::with_capacity(http_errors.len());
    for (http_error_key, count) in http_errors {
        let http_error_dict = PyDict::new(py);
        http_error_dict.set_item("name", &http_error_key.name)?;
        http_error_dict.set_item("code", http_error_key.code)?;
        http_error_dict.set_item("message", &http_error_key.msg)?;
        http_error_dict.set_item("url", &http_error_key.url)?;
        http_error_dict.set_item("host", &http_error_key.host)?;
        http_error_dict.set_item("path", &http_error_key.path)?;
        http_error_dict.set_item("source", &http_error_key.source)?;
        http_error_dict.set_item("count", count)?;
        http_errors_list.push(http_error_dict.unbind());
    }

    Ok(PyList::new(py, http_errors_list)?.unbind())
}
