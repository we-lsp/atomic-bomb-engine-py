use pyo3::types::PyDict;
use pyo3::{pyfunction, PyObject, PyResult, Python, ToPyObject};

#[pyfunction]
#[pyo3(signature=(
increase_step,
increase_interval,
))]
pub(crate) fn step_option(
    py: Python,
    increase_step: usize,
    increase_interval: usize,
) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    dict.set_item("increase_step", increase_step)?;
    dict.set_item("increase_interval", increase_interval)?;
    Ok(dict.to_object(py))
}
