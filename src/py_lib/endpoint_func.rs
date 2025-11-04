use pyo3::types::{PyAny, PyDict, PyDictMethods};
use pyo3::{pyfunction, Py, PyResult, Python};

#[pyfunction]
#[pyo3(signature=(
name,
url,
method,
weight,
json=None,
form_data=None,
multipart_options=None,
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
    weight: u32,
    json: Option<Py<PyAny>>,
    form_data: Option<Py<PyAny>>,
    multipart_options: Option<Py<PyAny>>,
    headers: Option<Py<PyAny>>,
    cookies: Option<String>,
    assert_options: Option<Py<PyAny>>,
    think_time_option: Option<Py<PyAny>>,
    setup_options: Option<Py<PyAny>>,
) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(py);
    dict.set_item("name", name)?;
    dict.set_item("url", url)?;
    dict.set_item("method", method)?;
    dict.set_item("weight", weight)?;
    if let Some(json) = json {
        dict.set_item("json", json)?;
    };
    if let Some(form_data) = form_data {
        dict.set_item("form_data", form_data)?;
    };
    if let Some(multipart_options) = multipart_options {
        dict.set_item("multipart_options", multipart_options)?;
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
    Ok(dict.into_any().unbind())
}
