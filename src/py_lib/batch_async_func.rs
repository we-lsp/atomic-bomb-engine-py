use pyo3::{PyAny, PyErr, pyfunction, PyResult, Python, ToPyObject};
use pyo3::types::{PyDict, PyList};
use pyo3_asyncio::tokio::future_into_py;
use crate::utils;

#[pyfunction]
#[pyo3(signature = (
test_duration_secs,
concurrent_requests,
api_endpoints,
step_option=None,
setup_options=None,
verbose=false,
should_prevent=false,
assert_channel_buffer_size=1024,
))]
pub(crate) fn batch_async<'a>(
    py: Python<'a>,
    test_duration_secs: u64,
    concurrent_requests: usize,
    api_endpoints: &'a PyList,
    step_option: Option<&PyDict>,
    setup_options: Option<&PyList>,
    verbose: bool,
    should_prevent: bool,
    assert_channel_buffer_size: usize,
) -> PyResult<&'a PyAny> {
    let endpoints = utils::parse_api_endpoints::new(py, api_endpoints)?;
    let step_opt = utils::parse_step_options::new(step_option)?;
    let setup_opts = utils::parse_setup_options::new(py, setup_options)?;

    future_into_py(py, async move {
        let result = atomic_bomb_engine::core::batch::batch(
            test_duration_secs,
            concurrent_requests,
            verbose,
            should_prevent,
            endpoints,
            step_opt,
            setup_opts,
            assert_channel_buffer_size
        ).await;

        Python::with_gil(|py| match result {
            Ok(test_result) => {
                let dict = PyDict::new(py);
                dict.set_item("total_duration", test_result.total_duration)?;
                dict.set_item("success_rate", test_result.success_rate)?;
                dict.set_item("error_rate", test_result.error_rate)?;
                dict.set_item("median_response_time", test_result.median_response_time)?;
                dict.set_item("response_time_95", test_result.response_time_95)?;
                dict.set_item("response_time_99", test_result.response_time_99)?;
                dict.set_item("total_requests", test_result.total_requests)?;
                dict.set_item("rps", test_result.rps)?;
                dict.set_item("max_response_time", test_result.max_response_time)?;
                dict.set_item("min_response_time", test_result.min_response_time)?;
                dict.set_item("err_count", test_result.err_count)?;
                dict.set_item("total_concurrent_number", test_result.total_concurrent_number)?;
                dict.set_item("total_data_kb", test_result.total_data_kb)?;
                dict.set_item("throughput_per_second_kb", test_result.throughput_per_second_kb)?;
                let http_error_list = utils::create_http_err_dict::create_http_error_dict(py, &test_result.http_errors)?;
                dict.set_item("http_errors", http_error_list)?;
                let assert_error_list = utils::create_assert_err_dict::create_assert_error_dict(py, &test_result.assert_errors)?;
                dict.set_item("assert_errors", assert_error_list)?;
                dict.set_item("timestamp", test_result.timestamp)?;
                let api_results = utils::create_api_results_dict::create_api_results_dict(py, test_result.api_results)?;
                dict.set_item("api_results", api_results)?;
                Ok(dict.to_object(py))
            },
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Error: {:?}", e))),
        })
    })
}
