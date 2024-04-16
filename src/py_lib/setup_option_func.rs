use pyo3::{pyfunction, PyObject, PyResult, Python};
use pyo3::types::{PyDict, PyList};

#[pyfunction]
pub(crate) fn setup_option(py: Python,
                           name: String,
                           url: String,
                           method: String,
                           timeout_secs: u64,
                           json: Option<PyObject>,
                           form_data: Option<PyObject>,
                           headers: Option<PyObject>,
                           cookies: Option<String>,
                           jsonpath_extract: Option<&PyList>
) -> PyResult<PyObject>{
    let dict = PyDict::new(py);
    dict.set_item("name", name)?;
    dict.set_item("url", url)?;
    dict.set_item("method", method)?;
    dict.set_item("timeout_secs", timeout_secs)?;
    if let Some(json) = json{
        dict.set_item("json", json)?;
    };
    if let Some(form_data) = form_data{
        dict.set_item("form_data", form_data)?;
    };
    if let Some(headers) = headers{
        dict.set_item("headers", headers)?;
    };
    if let Some(cookies) = cookies {
        dict.set_item("cookies", cookies)?;
    };
    if let Some(jsonpath_extract) = jsonpath_extract{
        dict.set_item("jsonpath_extract", jsonpath_extract)?;
    }
    Ok(dict.into())
}
