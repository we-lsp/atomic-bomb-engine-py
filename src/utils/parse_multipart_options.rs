use atomic_bomb_engine::models;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PyAnyMethods, PyDict, PyList, PyListMethods};

pub fn new(
    py: Python<'_>,
    py_multipart_options: Option<Py<PyList>>,
) -> PyResult<Option<Vec<models::multipart_option::MultipartOption>>> {
    match py_multipart_options {
        None => Ok(None),
        Some(ops_list) => {
            let list = ops_list.bind(py);
            let mut multipart_options: Vec<models::multipart_option::MultipartOption> =
                Vec::with_capacity(list.len());

            for item in list.iter() {
                let dict = item.downcast::<PyDict>()?;

                let form_key: String = dict
                    .get_item("form_key")?
                    .ok_or_else(|| PyErr::new::<PyRuntimeError, _>("form_key 不能为空".to_string()))?
                    .extract()?;

                let path: String = dict
                    .get_item("path")?
                    .ok_or_else(|| PyErr::new::<PyRuntimeError, _>("path 不能为空".to_string()))?
                    .extract()?;

                let file_name: String = dict
                    .get_item("file_name")?
                    .ok_or_else(|| PyErr::new::<PyRuntimeError, _>("file_name 不能为空".to_string()))?
                    .extract()?;

                let mime: String = dict
                    .get_item("mime")?
                    .ok_or_else(|| PyErr::new::<PyRuntimeError, _>("mime 不能为空".to_string()))?
                    .extract()?;

                multipart_options.push(models::multipart_option::MultipartOption {
                    form_key,
                    path,
                    file_name,
                    mime,
                });
            }

            Ok(Some(multipart_options))
        }
    }
}
