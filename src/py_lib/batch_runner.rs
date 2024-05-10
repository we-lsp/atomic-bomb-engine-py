use std::sync::Arc;
use atomic_bomb_engine::models::result::BatchResult;
use futures::stream::BoxStream;
use futures::StreamExt;
use crate::utils;
use pyo3::types::{PyDict, PyList};
use pyo3::{pyclass, pymethods, PyObject, PyRefMut, PyResult, Python, ToPyObject, PyAny};
use pyo3_asyncio::tokio::future_into_py;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

#[pyclass]
pub(crate) struct BatchRunner {
    runtime: Runtime,
    stream: Arc<Mutex<Option<BoxStream<'static, Result<Option<BatchResult>, anyhow::Error>>>>>,
}

#[pymethods]
impl BatchRunner {
    #[new]
    fn new() -> Self {
        BatchRunner {
            runtime: Runtime::new().unwrap(),
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
    fn run(&self,
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
           cookie_store_enable: bool,) -> PyResult<PyObject> {

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
            ).await;
            *stream_clone.lock().await = Some(stream);
            Ok::<(), pyo3::PyErr>(())
        };

        // 将Rust future转换为Python future并直接返回
        Python::with_gil(|py| {
            pyo3_asyncio::async_std::future_into_py(py, fut)
                .map(|py_any| py_any.to_object(py)) // Convert &PyAny to Py<PyAny>
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

        if let Some(stream) = stream_guard.as_mut() {
            let next_stream = slf.runtime.block_on(async {
                let n = stream.next().await;
                n
            });

            match next_stream {
                Some(Ok(result)) => {
                    let dict = PyDict::new(py);
                    Ok(Some(dict.to_object(py)))
                },
                Some(Err(e)) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
                None => Ok(None),
            }
        } else {
            Err(pyo3::exceptions::PyRuntimeError::new_err("Stream not initialized"))
        }
    }
}
