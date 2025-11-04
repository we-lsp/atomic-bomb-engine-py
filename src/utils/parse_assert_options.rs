use crate::utils::depythonize::depythonize;
use atomic_bomb_engine::models;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyListMethods};
use serde_json::Value;

pub fn _new(
    py: Python<'_>,
    assert_options: Option<Py<PyList>>,
) -> PyResult<Option<Vec<models::assert_option::AssertOption>>> {
    match assert_options {
        None => Ok(None),
        Some(ops_list) => {
            let list = ops_list.bind(py);
            let mut ops: Vec<models::assert_option::AssertOption> = Vec::with_capacity(list.len());
            for item in list.iter() {
                let dict = item.cast::<PyDict>()?;
                let jsonpath: String = dict
                    .get_item("jsonpath")?
                    .ok_or_else(|| {
                        PyErr::new::<PyRuntimeError, _>("必须输入一个jsonpath".to_string())
                    })?
                    .extract()?;

                let reference_any = dict.get_item("reference_object")?.ok_or_else(|| {
                    PyErr::new::<PyRuntimeError, _>("必须输入一个reference_object".to_string())
                })?;
                let reference_object: Value = depythonize(&reference_any)?;

                ops.push(models::assert_option::AssertOption {
                    jsonpath,
                    reference_object,
                });
            }
            Ok(Some(ops))
        }
    }
}
