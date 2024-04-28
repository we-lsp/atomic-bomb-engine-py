use pyo3::types::{PyDict, PyList};
use pyo3::{PyResult, Python};
use std::collections::HashMap;
use atomic_bomb_engine::models::http_error_stats::HttpErrKey;

pub fn create_http_error_dict<'py>(
    py: Python<'py>,
    http_errors: &HashMap<HttpErrKey, u32>,
) -> PyResult<&'py PyList> {
    if http_errors.is_empty() {
        return Ok(PyList::empty(py));
    }
    let mut http_errors_list = Vec::new();
    for (http_error_key, count) in http_errors {
        let http_error_key_clone = http_error_key.clone();
        let http_error_dict = PyDict::new(py);
        http_error_dict.set_item("name", http_error_key_clone.name)?;
        http_error_dict.set_item("code", http_error_key_clone.code)?;
        http_error_dict.set_item("message", http_error_key_clone.msg)?;
        http_error_dict.set_item("url", http_error_key_clone.url)?;
        http_error_dict.set_item("source", http_error_key_clone.source)?;
        http_error_dict.set_item("count", count)?;
        http_errors_list.push(http_error_dict)
    }
    Ok(PyList::new(py, http_errors_list))
}
