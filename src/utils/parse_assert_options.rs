use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde_json::Value;
use atomic_bomb_engine::models;
use serde_pyobject::from_pyobject;

pub fn new(
    py: Python,
    assert_options: Option<&PyList>,
) -> PyResult<Option<Vec<models::assert_option::AssertOption>>> {
    match assert_options {
        None => Ok(None),
        Some(ops_list) => {
            let mut ops: Vec<models::assert_option::AssertOption> = Vec::new();
            for item in ops_list.iter() {
                if let Ok(dict) = item.downcast::<PyDict>() {
                    let jsonpath: String = match dict.get_item("jsonpath") {
                        Ok(json_path_option) => match json_path_option {
                            None => {
                                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                                    "必须输入一个jsonpath".to_string(),
                                ))
                            }
                            Some(jsonpath) => jsonpath.to_string(),
                        },
                        Err(e) => {
                            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Error: {:?}",
                                e
                            )))
                        }
                    };

                    let reference_object: PyObject =
                        dict.get_item("reference_object").unwrap().to_object(py);
                    let reference_value: Value = match from_pyobject(reference_object.as_ref(py)) {
                        Ok(val) => val,
                        Err(e) => {
                            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Error: {:?}",
                                e
                            )))
                        }
                    };
                    ops.push(models::assert_option::AssertOption {
                        jsonpath,
                        reference_object: reference_value,
                    });
                }
            }
            Ok(Some(ops))
        }
    }
}
