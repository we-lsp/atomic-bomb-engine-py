use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde::de::DeserializeOwned;

pub fn depythonize<T>(value: &Bound<PyAny>) -> PyResult<T>
where
    T: DeserializeOwned,
{
    let py = value.py();
    let json_module = PyModule::import(py, "json")?;
    let dumps = json_module.getattr("dumps")?;
    let json_str: String = dumps.call1((value,))?.extract()?;
    serde_json::from_str(&json_str)
        .map_err(|err| PyErr::new::<PyRuntimeError, _>(format!("Error: {:?}", err)))
}
