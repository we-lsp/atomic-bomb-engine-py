use pyo3::types::{PyDict, PyList};
use pyo3::{pyfunction, PyObject, PyResult, Python};

#[pyfunction]
pub(crate) fn setup_option(
    py: Python,
    name: String,
    url: String,
    method: String,
    timeout_secs: u64,
    cookie_store_enable: bool,
    json: Option<PyObject>,
    form_data: Option<PyObject>,
    headers: Option<PyObject>,
    cookies: Option<String>,
    jsonpath_extract: Option<&PyList>,
) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    dict.set_item("name", name)?;
    dict.set_item("url", url)?;
    dict.set_item("method", method)?;
    dict.set_item("timeout_secs", timeout_secs)?;
    dict.set_item("cookie_store_enable", cookie_store_enable)?;
    if let Some(json) = json {
        dict.set_item("json", json)?;
    };
    if let Some(form_data) = form_data {
        dict.set_item("form_data", form_data)?;
    };
    if let Some(headers) = headers {
        dict.set_item("headers", headers)?;
    };
    if let Some(cookies) = cookies {
        dict.set_item("cookies", cookies)?;
    };
    if let Some(jsonpath_extract) = jsonpath_extract {
        dict.set_item("jsonpath_extract", jsonpath_extract)?;
    };
    Ok(dict.into())
}
