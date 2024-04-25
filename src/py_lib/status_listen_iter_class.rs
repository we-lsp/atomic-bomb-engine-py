use crate::utils;
use pyo3::types::PyDict;
use pyo3::{pyclass, pymethods, PyObject, PyRefMut, PyResult, Python, ToPyObject};
use tokio::runtime::Runtime;

#[pyclass]
pub(crate) struct StatusListenIter {}

#[pymethods]
impl StatusListenIter {
    #[new]
    fn new() -> Self {
        StatusListenIter {}
    }

    fn __iter__(slf: PyRefMut<Self>) -> PyResult<PyRefMut<Self>> {
        Ok(slf)
    }

    fn __next__(mut _slf: PyRefMut<Self>, py: Python) -> PyResult<Option<PyObject>> {
        let rt = Runtime::new().unwrap();
        let should_stop = rt.block_on(async {
            let lock = atomic_bomb_engine::core::status_share::SINGLE_SHOULD_STOP
                .lock()
                .await;
            *lock
        });
        if should_stop {
            return Ok(None); // 停止迭代
        }
        // let mut queue = atomic_bomb_engine::core::status_share::SINGLE_RESULT_QUEUE.lock();

        let mut queue = rt.block_on(async {
            let lock = atomic_bomb_engine::core::status_share::SINGLE_RESULT_QUEUE
                .lock()
                .await;
            lock
        });
        if let Some(test_result) = queue.pop_front() {
            let dict = PyDict::new(py);
            dict.set_item("total_duration", test_result.total_duration)?;
            dict.set_item("success_rate", test_result.success_rate)?;
            dict.set_item("median_response_time", test_result.median_response_time)?;
            dict.set_item("response_time_95", test_result.response_time_95)?;
            dict.set_item("response_time_99", test_result.response_time_99)?;
            dict.set_item("total_requests", test_result.total_requests)?;
            dict.set_item("rps", test_result.rps)?;
            dict.set_item("max_response_time", test_result.max_response_time)?;
            dict.set_item("min_response_time", test_result.min_response_time)?;
            dict.set_item("err_count", test_result.err_count)?;
            dict.set_item("total_data_kb", test_result.total_data_kb)?;
            dict.set_item(
                "throughput_per_second_kb",
                test_result.throughput_per_second_kb,
            )?;
            let http_error_list =
                utils::create_http_err_dict::create_http_error_dict(py, &test_result.http_errors)?;
            dict.set_item("http_errors", http_error_list)?;
            let assert_error_list = utils::create_assert_err_dict::create_assert_error_dict(
                py,
                &test_result.assert_errors,
            )?;
            dict.set_item("assert_errors", assert_error_list)?;
            dict.set_item("timestamp", test_result.timestamp)?;
            Ok(Some(dict.to_object(py)))
        } else {
            Ok(Some(py.None())) // 暂时没有消息
        }
    }
}
