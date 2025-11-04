use std::collections::HashMap;

use atomic_bomb_engine::models;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PyAnyMethods, PyDict, PyList, PyListMethods};
use pythonize::depythonize;
use serde_json::Value;

pub fn new(
    py: Python<'_>,
    py_setup_options: Option<Py<PyList>>,
) -> PyResult<Option<Vec<models::setup::SetupApiEndpoint>>> {
    match py_setup_options {
        None => Ok(None),
        Some(options) => {
            let list = options.bind(py);
            let mut setup_options: Vec<models::setup::SetupApiEndpoint> =
                Vec::with_capacity(list.len());

            for item in list.iter() {
                let dict = item.downcast::<PyDict>()?;

                let name: String = dict
                    .get_item("name")?
                    .ok_or_else(|| PyErr::new::<PyRuntimeError, _>("name不能为空".to_string()))?
                    .extract()?;

                let url: String = dict
                    .get_item("url")?
                    .ok_or_else(|| PyErr::new::<PyRuntimeError, _>("url不能为空".to_string()))?
                    .extract()?;

                let method: String = dict
                    .get_item("method")?
                    .ok_or_else(|| PyErr::new::<PyRuntimeError, _>("method不能为空".to_string()))?
                    .extract()?;

                let json: Option<Value> = dict
                    .get_item("json")?
                    .map(|value| depythonize(&value).map_err(|e| {
                        PyErr::new::<PyRuntimeError, _>(format!("Error: {:?}", e))
                    }))
                    .transpose()?;

                let form_data: Option<HashMap<String, String>> = dict
                    .get_item("form_data")?
                    .map(|value| depythonize(&value).map_err(|e| {
                        PyErr::new::<PyRuntimeError, _>(format!("Error: {:?}", e))
                    }))
                    .transpose()?;

                let headers: Option<HashMap<String, String>> = dict
                    .get_item("headers")?
                    .map(|value| depythonize(&value).map_err(|e| {
                        PyErr::new::<PyRuntimeError, _>(format!("Error: {:?}", e))
                    }))
                    .transpose()?;

                let cookies: Option<String> = dict
                    .get_item("cookies")?
                    .map(|value| depythonize(&value).map_err(|e| {
                        PyErr::new::<PyRuntimeError, _>(format!("Error: {:?}", e))
                    }))
                    .transpose()?;

                let jsonpath_extract: Option<Vec<models::setup::JsonpathExtract>> = dict
                    .get_item("jsonpath_extract")?
                    .map(|value| depythonize(&value).map_err(|e| {
                        PyErr::new::<PyRuntimeError, _>(format!("Error: {:?}", e))
                    }))
                    .transpose()?;

                let multipart_options: Option<Vec<models::multipart_option::MultipartOption>> = dict
                    .get_item("multipart_options")?
                    .map(|value| depythonize(&value).map_err(|e| {
                        PyErr::new::<PyRuntimeError, _>(format!("Error: {:?}", e))
                    }))
                    .transpose()?;

                setup_options.push(models::setup::SetupApiEndpoint {
                    name,
                    url,
                    method,
                    json,
                    form_data,
                    multipart_options,
                    headers,
                    cookies,
                    jsonpath_extract,
                });
            }

            Ok(Some(setup_options))
        }
    }
}
