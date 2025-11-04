use crate::utils;
use crate::utils::depythonize::depythonize;
use std::collections::HashMap;

use atomic_bomb_engine::models;
use atomic_bomb_engine::models::api_endpoint::ThinkTime;
use atomic_bomb_engine::models::assert_option::AssertOption;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyListMethods};
use serde_json::Value;

pub fn new(
    py: Python<'_>,
    api_endpoints: Py<PyList>,
) -> PyResult<Vec<models::api_endpoint::ApiEndpoint>> {
    let mut endpoints: Vec<models::api_endpoint::ApiEndpoint> = Vec::new();
    let bound_list = api_endpoints.bind(py);
    for item in bound_list.iter() {
        let dict = item.cast::<PyDict>()?;

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

        let weight: u32 = dict
            .get_item("weight")?
            .ok_or_else(|| PyErr::new::<PyRuntimeError, _>("weight不能为空".to_string()))?
            .extract()?;

        let json: Option<Value> = dict
            .get_item("json")?
            .map(|value| depythonize(&value))
            .transpose()?;

        let form_data: Option<HashMap<String, String>> = dict
            .get_item("form_data")?
            .map(|value| depythonize(&value))
            .transpose()?;

        let headers: Option<HashMap<String, String>> = dict
            .get_item("headers")?
            .map(|value| depythonize(&value))
            .transpose()?;

        let cookies: Option<String> = dict
            .get_item("cookies")?
            .map(|value| depythonize(&value))
            .transpose()?;

        let assert_options: Option<Vec<AssertOption>> = dict
            .get_item("assert_options")?
            .map(|value| depythonize(&value))
            .transpose()?;

        let think_time_option: Option<ThinkTime> = dict
            .get_item("think_time_option")?
            .map(|value| depythonize(&value))
            .transpose()?;

        let setup_options_py = match dict.get_item("setup_options")? {
            Some(value) => Some(value.extract::<Py<PyList>>()?),
            None => None,
        };

        let setup_options = utils::parse_setup_options::new(py, setup_options_py)?;

        let multipart_options_py = match dict.get_item("multipart_options")? {
            Some(value) => Some(value.extract::<Py<PyList>>()?),
            None => None,
        };

        let multipart_options =
            utils::parse_multipart_options::new(py, multipart_options_py)?;

        endpoints.push(models::api_endpoint::ApiEndpoint {
            name,
            url,
            method,
            weight,
            json,
            form_data,
            multipart_options,
            headers,
            cookies,
            assert_options,
            think_time_option,
            setup_options,
        });
    }
    Ok(endpoints)
}
