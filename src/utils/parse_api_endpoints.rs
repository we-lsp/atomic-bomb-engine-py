use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde_json::Value;
use atomic_bomb_engine::models;
use atomic_bomb_engine::models::assert_option::AssertOption;
use serde_pyobject::from_pyobject;

pub fn new(
    py: Python,
    api_endpoints: &PyList,
) -> PyResult<Vec<models::api_endpoint::ApiEndpoint>> {
    let mut endpoints: Vec<models::api_endpoint::ApiEndpoint> = Vec::new();
    for item in api_endpoints.iter(){
        if let Ok(dict) = item.downcast::<PyDict>() {
            let name: String = match dict.get_item("name") {
                Ok(name) => match name {
                    None => {
                        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("name不能为空".to_string()))
                    }
                    Some(name) => name.to_string(),
                },
                Err(e) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Error: {:?}", e)))
                }
            };

            let url: String = match dict.get_item("url") {
                Ok(url) => match url {
                    None => {
                        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("url不能为空".to_string()))
                    }
                    Some(url) => url.to_string(),
                },
                Err(e) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Error: {:?}", e)))
                }
            };

            let method: String = match dict.get_item("method") {
                Ok(method) => match method {
                    None => {
                        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("method不能为空".to_string()))
                    }
                    Some(method) => method.to_string(),
                },
                Err(e) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Error: {:?}", e)))
                }
            };

            let timeout_secs: u64 = match dict.get_item("timeout_secs") {
                Ok(timeout_secs) => match timeout_secs {
                    None => {
                        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("timeout_secs不能为空".to_string()))
                    }
                    Some(timeout_secs) => timeout_secs.to_string().parse()?,
                },
                Err(e) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Error: {:?}", e)))
                }
            };

            let weight: u32 = match dict.get_item("weight") {
                Ok(weight) => match weight {
                    None => {
                        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("weight不能为空".to_string()))
                    }
                    Some(weight) => weight.to_string().parse()?,
                },
                Err(e) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Error: {:?}", e)))
                }
            };

            let json_obj: PyObject = dict.get_item("json").unwrap().to_object(py);
            let json: Option<Value> = match from_pyobject(json_obj.as_ref(py)) {
                Ok(val) => val,
                Err(e) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Error: {:?}", e)))
                }
            };

            let form_data_obj: PyObject = dict.get_item("form_data").unwrap().to_object(py);
            let form_data:Option<HashMap<String, String>> = match from_pyobject(form_data_obj.as_ref(py)) {
                Ok(val) => val,
                Err(e) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Error: {:?}", e)))
                }
            };

            let headers_obj: PyObject = dict.get_item("headers").unwrap().to_object(py);
            let headers:Option<HashMap<String, String>> = match from_pyobject(headers_obj.as_ref(py)) {
                Ok(val) => val,
                Err(e) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Error: {:?}", e)))
                }
            };

            let cookies_obj: PyObject = dict.get_item("cookies").unwrap().to_object(py);
            let cookies:Option<String> = match from_pyobject(cookies_obj.as_ref(py)) {
                Ok(val) => val,
                Err(e) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Error: {:?}", e)))
                }
            };

            let assert_options: Option<Vec<AssertOption>> = match dict.get_item("assert_options"){
                Ok(op_py_any) => {
                    match op_py_any {
                        None => {None}
                        Some(py_any) => {
                            let pyobj = py_any.to_object(py);
                            match from_pyobject(pyobj.as_ref(py)) {
                                Ok(val) => val,
                                Err(e) =>{
                                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                        "Error: {:?}",
                                        e
                                    )))
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Error: {:?}", e)))
                }
            };


            endpoints.push(models::api_endpoint::ApiEndpoint {
                name,
                url,
                method,
                timeout_secs,
                weight,
                json,
                form_data,
                headers,
                cookies,
                assert_options,
            });
        }
    }
    Ok(endpoints)
}

