use pyo3::{pyclass, PyErr, pymethods, PyObject, PyRefMut, PyResult, Python, ToPyObject};
use pyo3::types::PyDict;
use crate::utils;
use tokio::runtime::Runtime;


#[pyclass]
pub(crate) struct BatchListenIter {}

#[pymethods]
impl BatchListenIter {
    #[new]
    fn new() -> Self {
        BatchListenIter {}
    }

    fn __iter__(slf: PyRefMut<Self>) -> PyResult<PyRefMut<Self>> {
        Ok(slf)
    }

    fn __next__(_slf: PyRefMut<Self>, py: Python) -> PyResult<Option<PyObject>> {
        // let should_stop = *atomic_bomb_engine::core::status_share::RESULTS_SHOULD_STOP.lock();
        // if should_stop {
        //     return  Err(PyErr::new::<pyo3::exceptions::PyStopIteration, _>("No more data available"));
        // }
        let rt = Runtime::new().unwrap();
        let should_stop = rt.block_on(async {
            let lock = atomic_bomb_engine::core::status_share::RESULTS_SHOULD_STOP.lock().await;
            *lock
        });
        if should_stop {
            return  Err(PyErr::new::<pyo3::exceptions::PyStopIteration, _>("No more data available"));
        }
        // let mut queue = atomic_bomb_engine::core::status_share::RESULTS_QUEUE.lock();
        let mut queue = rt.block_on(async {
            let lock = atomic_bomb_engine::core::status_share::RESULTS_QUEUE.lock().await;
            lock
        });
        if let Some(test_result) = queue.pop_front() {
            if test_result.total_data_kb == 0.0 ||
                // 如果有名字为空
                test_result.api_results.iter().any(|api_result| api_result.name.is_empty())
            {
              return   Ok(Some(py.None()))
            };
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
            dict.set_item("total_data_kb", test_result.total_data_kb)?;
            dict.set_item("throughput_per_second_kb", test_result.throughput_per_second_kb)?;
            let http_error_list = utils::create_http_err_dict::create_http_error_dict(py, &test_result.http_errors)?;
            dict.set_item("http_errors", http_error_list)?;
            let assert_error_list = utils::create_assert_err_dict::create_assert_error_dict(py, &test_result.assert_errors)?;
            dict.set_item("assert_errors", assert_error_list)?;
            dict.set_item("timestamp", test_result.timestamp)?;
            let api_results = utils::create_api_results_dict::create_api_results_dict(py, test_result.api_results)?;
            dict.set_item("api_results", api_results)?;
            dict.set_item("total_concurrent_number", test_result.total_concurrent_number)?;
            Ok(Some(dict.to_object(py)))
        } else {
            Ok(Some(py.None()))
        }
    }
}
