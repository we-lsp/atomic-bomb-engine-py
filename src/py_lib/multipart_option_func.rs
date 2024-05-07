use pyo3::types::PyDict;
use pyo3::{pyfunction, PyObject, PyResult, Python};

#[pyfunction]
#[pyo3(signature=(
form_key,
path,
file_name,
mime,
))]
pub(crate) fn multipart_option(
    py: Python,
    form_key: String,
    path: String,
    file_name: String,
    mime: String,
) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    dict.set_item("form_key", form_key)?;
    dict.set_item("path", path)?;
    dict.set_item("file_name", file_name)?;
    dict.set_item("mime", mime)?;
    Ok(dict.into())
}
