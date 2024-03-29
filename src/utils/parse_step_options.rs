use pyo3::prelude::*;
use pyo3::types::{PyDict};
use atomic_bomb_engine::models;

pub fn new(
    step_option: Option<&PyDict>,
) -> PyResult<Option<models::step_option::StepOption>> {
    match step_option {
        None => Ok(None),
        Some(ops_dict) => {
            let increase_step: usize = match ops_dict.get_item("increase_step") {
                Ok(increase_step_py_any) => {
                    match increase_step_py_any {
                        None => {
                            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                                "必须输入一个increase_step".to_string(),))
                        }
                        Some(increase_step) => {
                            let result: PyResult<usize> = increase_step.extract::<usize>();
                            match result{
                                Ok(res) => {res}
                                Err(e) => {
                                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                                        format!("increase_step必须是一个整数::{:?}", e),
                                    ))
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Error: {:?}",
                        e
                    )))
                }
            };

            let increase_interval: u64 = match ops_dict.get_item("increase_interval") {
                Ok(increase_interval_py_any) => {
                    match increase_interval_py_any{
                        None => {
                            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                                "必须输入一个increase_interval".to_string(),))
                        }
                        Some(increase_interval) => {
                            let result: PyResult<u64> = increase_interval.extract::<u64>();
                            match result {
                                Ok(res) => {res}
                                Err(e) => {
                                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                                        format!("increase_interval必须是一个整数::{:?}", e),
                                    ))
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Error: {:?}",
                        e
                    )))
                }
            };

            let step_opt = models::step_option::StepOption{ increase_step, increase_interval };
            Ok(Some(step_opt))
        }
    }
}
