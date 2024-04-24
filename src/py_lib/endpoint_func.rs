use pyo3::types::{PyDict, PyList};
use pyo3::{pyfunction, PyObject, PyResult, Python};

#[pyfunction]
#[pyo3(signature=(
name,
url,
method,
timeout_secs,
weight,
json=None,
form_data=None,
headers=None,
cookies=None,
assert_options=None,
think_time_option=None,
setup_options=None,
))]
pub(crate) fn endpoint(
    py: Python,
    name: String,
    url: String,
    method: String,
    timeout_secs: u64,
    weight: u32,
    json: Option<PyObject>,
    form_data: Option<PyObject>,
    headers: Option<PyObject>,
    cookies: Option<String>,
    assert_options: Option<&PyList>,
    think_time_option: Option<PyObject>,
    setup_options: Option<&PyList>,
) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    dict.set_item("name", name)?;
    dict.set_item("url", url)?;
    dict.set_item("method", method)?;
    dict.set_item("timeout_secs", timeout_secs)?;
    dict.set_item("weight", weight)?;
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
    if let Some(assert_options) = assert_options {
        dict.set_item("assert_options", assert_options)?;
    };
    if let Some(think_time_option) = think_time_option {
        dict.set_item("think_time_option", think_time_option)?;
    };
    if let Some(setup_options) = setup_options {
        dict.set_item("setup_options", setup_options)?;
    }
    Ok(dict.into())
}
