use pyo3::prelude::*;
mod py_lib;
mod utils;

#[pymodule]
#[pyo3(name = "atomic_bomb_engine")]
fn atomic_bomb_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(
        py_lib::assert_option_func::assert_option,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(py_lib::endpoint_func::endpoint, m)?)?;
    m.add_function(wrap_pyfunction!(py_lib::step_option_func::step_option, m)?)?;
    m.add_function(wrap_pyfunction!(
        py_lib::setup_option_func::setup_option,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        py_lib::jsonpath_extract_func::jsonpath_extract_option,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        py_lib::think_time_option_func::think_time_option,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        py_lib::multipart_option_func::multipart_option,
        m
    )?)?;
    m.add_class::<py_lib::batch_runner::BatchRunner>()?;
    Ok(())
}
