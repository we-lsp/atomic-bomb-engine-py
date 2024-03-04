use std::collections::HashMap;
use pyo3::{PyResult, Python};
use pyo3::types::PyDict;
pub fn create_http_error_dict<'py>(py: Python<'py>, http_errors: &HashMap<(u16, String, String), u32>) -> PyResult<&'py PyDict> {
    if http_errors.is_empty() {
        return Ok(PyDict::new(py));
    }

    let http_error_dict = PyDict::new(py);
    for ((code, message, url), count) in http_errors {
        let key = format!("{}|{}", code, message);
        http_error_dict.set_item("code", code)?;
        http_error_dict.set_item("message", message)?;
        http_error_dict.set_item("count", count)?;
        http_error_dict.set_item("url", url)?;
    }
    Ok(http_error_dict)
}
