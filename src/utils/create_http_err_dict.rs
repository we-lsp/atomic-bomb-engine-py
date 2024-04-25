use pyo3::types::{PyDict, PyList};
use pyo3::{PyResult, Python};
use std::collections::HashMap;
pub fn create_http_error_dict<'py>(
    py: Python<'py>,
    http_errors: &HashMap<(u16, String, String), u32>,
) -> PyResult<&'py PyList> {
    if http_errors.is_empty() {
        return Ok(PyList::empty(py));
    }

    let mut http_errors_list = Vec::new();
    for ((code, message, url), count) in http_errors {
        let http_error_dict = PyDict::new(py);

        http_error_dict.set_item("code", code)?;
        http_error_dict.set_item("message", message)?;
        http_error_dict.set_item("count", count)?;
        http_error_dict.set_item("url", url)?;

        http_errors_list.push(http_error_dict)
    }
    Ok(PyList::new(py, http_errors_list))
}
