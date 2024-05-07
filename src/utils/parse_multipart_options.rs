use atomic_bomb_engine::models;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

pub fn new(
    _py: Python,
    py_multipart_options: Option<&PyList>,
) -> PyResult<Option<Vec<models::multipart_option::MultipartOption>>> {
    return match py_multipart_options {
        None => Ok(None),
        Some(ops) => {
            let mut multipart_options: Vec<models::multipart_option::MultipartOption> = Vec::new();
            for item in ops.iter() {
                if let Ok(dict) = item.downcast::<PyDict>() {
                    let form_key: String = match dict.get_item("form_key") {
                        Ok(form_key) => match form_key {
                            None => {
                                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                                    "form_key".to_string(),
                                ))
                            }
                            Some(form_key) => form_key.to_string(),
                        },
                        Err(e) => {
                            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Error: {:?}",
                                e
                            )))
                        }
                    };

                    let path: String = match dict.get_item("path") {
                        Ok(path) => match path {
                            None => {
                                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                                    "path".to_string(),
                                ))
                            }
                            Some(path) => path.to_string(),
                        },
                        Err(e) => {
                            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Error: {:?}",
                                e
                            )))
                        }
                    };

                    let file_name: String = match dict.get_item("file_name") {
                        Ok(file_name) => match file_name {
                            None => {
                                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                                    "file_name".to_string(),
                                ))
                            }
                            Some(file_name) => file_name.to_string(),
                        },
                        Err(e) => {
                            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Error: {:?}",
                                e
                            )))
                        }
                    };

                    let mime: String = match dict.get_item("mime") {
                        Ok(mime) => match mime {
                            None => {
                                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                                    "mime".to_string(),
                                ))
                            }
                            Some(mime) => mime.to_string(),
                        },
                        Err(e) => {
                            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Error: {:?}",
                                e
                            )))
                        }
                    };

                    multipart_options.push(models::multipart_option::MultipartOption {
                        form_key,
                        path,
                        file_name,
                        mime,
                    })
                }
            }
            Ok(Option::from(multipart_options))
        }
    };
}
