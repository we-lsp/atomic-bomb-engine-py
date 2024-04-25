use atomic_bomb_engine::models;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde_json::Value;
use serde_pyobject::from_pyobject;
use std::collections::HashMap;

pub fn new(
    py: Python,
    py_setup_options: Option<&PyList>,
) -> PyResult<Option<Vec<models::setup::SetupApiEndpoint>>> {
    return match py_setup_options {
        None => Ok(None),
        Some(ops) => {
            let mut setup_options: Vec<models::setup::SetupApiEndpoint> = Vec::new();
            for item in ops.iter() {
                if let Ok(dict) = item.downcast::<PyDict>() {
                    let name: String = match dict.get_item("name") {
                        Ok(name) => match name {
                            None => {
                                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                                    "name不能为空".to_string(),
                                ))
                            }
                            Some(name) => name.to_string(),
                        },
                        Err(e) => {
                            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Error: {:?}",
                                e
                            )))
                        }
                    };

                    let url: String = match dict.get_item("url") {
                        Ok(url) => match url {
                            None => {
                                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                                    "url不能为空".to_string(),
                                ))
                            }
                            Some(url) => url.to_string(),
                        },
                        Err(e) => {
                            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Error: {:?}",
                                e
                            )))
                        }
                    };

                    let method: String = match dict.get_item("method") {
                        Ok(method) => match method {
                            None => {
                                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                                    "method不能为空".to_string(),
                                ))
                            }
                            Some(method) => method.to_string(),
                        },
                        Err(e) => {
                            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Error: {:?}",
                                e
                            )))
                        }
                    };

                    let json_obj: PyObject = dict.get_item("json").unwrap().to_object(py);
                    let json: Option<Value> = match from_pyobject(json_obj.as_ref(py)) {
                        Ok(val) => val,
                        Err(e) => {
                            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Error: {:?}",
                                e
                            )))
                        }
                    };

                    let form_data_obj: PyObject = dict.get_item("form_data").unwrap().to_object(py);
                    let form_data: Option<HashMap<String, String>> =
                        match from_pyobject(form_data_obj.as_ref(py)) {
                            Ok(val) => val,
                            Err(e) => {
                                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                                    format!("Error: {:?}", e),
                                ))
                            }
                        };

                    let headers_obj: PyObject = dict.get_item("headers").unwrap().to_object(py);
                    let headers: Option<HashMap<String, String>> =
                        match from_pyobject(headers_obj.as_ref(py)) {
                            Ok(val) => val,
                            Err(e) => {
                                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                                    format!("Error: {:?}", e),
                                ))
                            }
                        };

                    let cookies_obj: PyObject = dict.get_item("cookies").unwrap().to_object(py);
                    let cookies: Option<String> = match from_pyobject(cookies_obj.as_ref(py)) {
                        Ok(val) => val,
                        Err(e) => {
                            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Error: {:?}",
                                e
                            )))
                        }
                    };

                    let jsonpath_extract: Option<Vec<models::setup::JsonpathExtract>> = match dict
                        .get_item("jsonpath_extract")
                    {
                        Ok(op_py_any) => match op_py_any {
                            None => None,
                            Some(py_any) => {
                                let pyobj = py_any.to_object(py);
                                match from_pyobject(pyobj.as_ref(py)) {
                                    Ok(val) => val,
                                    Err(e) => {
                                        return Err(
                                            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                                                format!("Error: {:?}", e),
                                            ),
                                        )
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Error: {:?}",
                                e
                            )))
                        }
                    };

                    setup_options.push(models::setup::SetupApiEndpoint {
                        name,
                        url,
                        method,
                        json,
                        form_data,
                        headers,
                        cookies,
                        jsonpath_extract,
                    })
                }
            }
            Ok(Option::from(setup_options))
        }
    };
}
