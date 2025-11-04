use atomic_bomb_engine::models::result::ApiResult;
use pyo3::types::{PyDict, PyDictMethods, PyList};
use pyo3::{Py, PyResult, Python};

pub fn create_api_results_dict(py: Python<'_>, api_results: Vec<ApiResult>) -> PyResult<Py<PyList>> {
    if api_results.is_empty() {
        return Ok(PyList::empty(py).unbind());
    }

    let mut results: Vec<Py<PyDict>> = Vec::with_capacity(api_results.len());

    for result in api_results {
        let res_dict = PyDict::new(py);

        res_dict.set_item("name", result.name)?;
        res_dict.set_item("url", result.url)?;
        res_dict.set_item("host", result.host)?;
        res_dict.set_item("path", result.path)?;
        res_dict.set_item("method", result.method)?;
        res_dict.set_item("success_rate", result.success_rate)?;
        res_dict.set_item("error_rate", result.error_rate)?;
        res_dict.set_item("median_response_time", result.median_response_time)?;
        res_dict.set_item("response_time_95", result.response_time_95)?;
        res_dict.set_item("response_time_99", result.response_time_99)?;
        res_dict.set_item("total_requests", result.total_requests)?;
        res_dict.set_item("rps", result.rps)?;
        res_dict.set_item("max_response_time", result.max_response_time)?;
        res_dict.set_item("min_response_time", result.min_response_time)?;
        res_dict.set_item("err_count", result.err_count)?;
        res_dict.set_item("total_data_kb", result.total_data_kb)?;
        res_dict.set_item("throughput_per_second_kb", result.throughput_per_second_kb)?;
        res_dict.set_item("concurrent_number", result.concurrent_number)?;

        results.push(res_dict.unbind());
    }

    Ok(PyList::new(py, results)?.unbind())
}
