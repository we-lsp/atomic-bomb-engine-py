use crate::utils;
use atomic_bomb_engine::models::result::BatchResult;
use futures::stream::BoxStream;
use futures::StreamExt;
use pyo3::types::{PyDict, PyList};
use pyo3::{pyclass, pymethods, PyObject, PyRefMut, PyResult, Python, ToPyObject};
use std::sync::Arc;
use tokio::sync::Mutex;

#[pyclass]
pub(crate) struct BatchRunner {
    runtime: tokio::runtime::Runtime,
    stream: Arc<Mutex<Option<BoxStream<'static, Result<Option<BatchResult>, anyhow::Error>>>>>,
}

#[pymethods]
impl BatchRunner {
    #[new]
    fn new() -> Self {
        BatchRunner {
            runtime: tokio::runtime::Runtime::new().unwrap(),
            stream: Arc::new(Mutex::new(None)),
        }
    }

    #[pyo3(signature = (
    test_duration_secs,
    concurrent_requests,
    api_endpoints,
    step_option=None,
    setup_options=None,
    verbose=false,
    should_prevent=false,
    assert_channel_buffer_size=1024,
    timeout_secs=0,
    cookie_store_enable=true,
    ))]
    fn run(
        &self,
        py: Python,
        test_duration_secs: u64,
        concurrent_requests: usize,
        api_endpoints: &PyList,
        step_option: Option<&PyDict>,
        setup_options: Option<&PyList>,
        verbose: bool,
        should_prevent: bool,
        assert_channel_buffer_size: usize,
        timeout_secs: u64,
        cookie_store_enable: bool,
    ) -> PyResult<PyObject> {
        let stream_clone = self.stream.clone();
        let endpoints = utils::parse_api_endpoints::new(py, api_endpoints)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        let step_opt = utils::parse_step_options::new(step_option)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        let setup_opts = utils::parse_setup_options::new(py, setup_options)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        let fut = async move {
            let stream = atomic_bomb_engine::core::run_batch::run_batch(
                test_duration_secs,
                concurrent_requests,
                timeout_secs,
                cookie_store_enable,
                verbose,
                should_prevent,
                endpoints,
                step_opt,
                setup_opts,
                assert_channel_buffer_size,
            )
            .await;
            *stream_clone.lock().await = Some(stream);
            Ok::<(), pyo3::PyErr>(())
        };

        Python::with_gil(|py| {
            pyo3_asyncio::tokio::future_into_py(py, fut).map(|py_any| py_any.to_object(py))
        })
    }

    fn __iter__(slf: PyRefMut<Self>) -> PyResult<PyRefMut<Self>> {
        Ok(slf)
    }

    fn __next__(slf: PyRefMut<'_, Self>, py: Python) -> PyResult<Option<PyObject>> {
        let mut stream_guard = slf.runtime.block_on(async {
            let stream = slf.stream.lock().await;
            stream
        });

        match stream_guard.as_mut() {
            Some(stream) => {
                let next_stream = slf.runtime.block_on(async {
                    let n = stream.next().await;
                    n
                });

                match next_stream {
                    Some(Ok(result)) => {
                        let dict = PyDict::new(py);
                        if let Some(test_result) = result {
                            dict.set_item("total_duration", test_result.total_duration)?;
                            dict.set_item("success_rate", test_result.success_rate)?;
                            dict.set_item("error_rate", test_result.error_rate)?;
                            dict.set_item(
                                "median_response_time",
                                test_result.median_response_time,
                            )?;
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
                                utils::create_http_err_dict::create_http_error_dict(
                                    py,
                                    &test_result.http_errors,
                                )?;
                            dict.set_item("http_errors", http_error_list)?;
                            let assert_error_list =
                                utils::create_assert_err_dict::create_assert_error_dict(
                                    py,
                                    &test_result.assert_errors,
                                )?;
                            dict.set_item("assert_errors", assert_error_list)?;
                            dict.set_item("timestamp", test_result.timestamp)?;
                            let api_results =
                                utils::create_api_results_dict::create_api_results_dict(
                                    py,
                                    test_result.api_results,
                                )?;
                            dict.set_item("api_results", api_results)?;
                            dict.set_item(
                                "total_concurrent_number",
                                test_result.total_concurrent_number,
                            )?;
                            dict.set_item("errors_per_second", test_result.errors_per_second)?;
                        };
                        Ok(Some(dict.to_object(py)))
                    }
                    Some(Err(e)) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
                    None => Ok(None),
                }
            }
            None => {
                eprintln!("stream未初始化，请等待");
                let dict = PyDict::new(py);
                dict.set_item("should_wait", true)?;
                Ok(Some(dict.to_object(py)))
            }
        }
    }
}
