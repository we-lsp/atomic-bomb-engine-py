use pyo3::types::{PyAny, PyDict, PyDictMethods};
use pyo3::{pyfunction, Py, PyResult, Python};

#[pyfunction]
#[pyo3(signature=(
increase_step,
increase_interval,
))]
pub(crate) fn step_option(
    py: Python,
    increase_step: usize,
    increase_interval: usize,
) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(py);
    dict.set_item("increase_step", increase_step)?;
    dict.set_item("increase_interval", increase_interval)?;
    Ok(dict.into_any().unbind())
}
