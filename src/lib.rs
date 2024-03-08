use pyo3::prelude::*;
use tokio;
use pyo3::types::{PyDict, PyAny, PyList};
use tokio::runtime::Runtime;
use ::atomic_bomb_engine as abe;
use abe::{core};
use pyo3_asyncio::tokio::future_into_py;
use pyo3_asyncio;
use serde_json;


mod utils;

#[pyfunction]
#[pyo3(signature = (
url,
method,
test_duration_secs = 1,
concurrent_requests = 1,
timeout_secs = 30,
verbose = false,
json_str=None,
form_data_str=None,
headers=None,
cookie=None,
should_prevent=false,
assert_options=None)
)]
fn run(
    py: Python,
    url: String,
    method: String,
    test_duration_secs: u64,
    concurrent_requests: i32,
    timeout_secs: u64,
    verbose: bool,
    json_str: Option<String>,
    form_data_str: Option<String>,
    headers: Option<Vec<String>>,
    cookie: Option<String>,
    should_prevent:bool,
    assert_options: Option<&PyList>
) -> PyResult<PyObject> {
    let options = utils::parse_assert_options::new(py, assert_options)?;
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(async move {
        core::execute::run(
            &url,
            test_duration_secs,
            concurrent_requests,
            timeout_secs,
            verbose,
            &method,
            json_str,
            form_data_str,
            headers,
            cookie,
            should_prevent,
            options
        ).await
    });

    match result {
        Ok(test_result) => {
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
            dict.set_item("throughput_per_second_kb", test_result.throughput_per_second_kb)?;
            let http_error_list = utils::create_http_err_dict::create_http_error_dict(py, &test_result.http_errors)?;
            dict.set_item("http_errors", http_error_list)?;
            let assert_error_list = utils::create_assert_err_dict::create_assert_error_dict(py, &test_result.assert_errors)?;
            dict.set_item("assert_errors", assert_error_list)?;
            dict.set_item("timestamp", test_result.timestamp)?;
            Ok(dict.into())
        },
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Error: {:?}", e))),
    }
}

#[pyfunction]
#[pyo3(signature = (
url,
method,
test_duration_secs = 1,
concurrent_requests = 1,
timeout_secs = 30,
verbose = false,
json_str=None,
form_data_str=None,
headers=None,
cookie=None,
should_prevent=false,
assert_options=None,
))]
fn run_async<'a>(
    py: Python<'a>,
    url: String,
    method: String,
    test_duration_secs: u64,
    concurrent_requests: i32,
    timeout_secs: u64,
    verbose: bool,
    json_str: Option<String>,
    form_data_str: Option<String>,
    headers: Option<Vec<String>>,
    cookie: Option<String>,
    should_prevent: bool,
    assert_options: Option<&'a PyList>,
) -> PyResult<&'a PyAny> {
    let options = utils::parse_assert_options::new(py, assert_options)?;
    future_into_py(py, async move {
        let result = core::execute::run(
            &url,
            test_duration_secs,
            concurrent_requests,
            timeout_secs,
            verbose,
            &method,
            json_str,
            form_data_str,
            headers,
            cookie,
            should_prevent,
            options
        ).await;

        Python::with_gil(|py| match result {
            Ok(test_result) => {
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
                dict.set_item("throughput_per_second_kb", test_result.throughput_per_second_kb)?;
                let http_error_list = utils::create_http_err_dict::create_http_error_dict(py, &test_result.http_errors)?;
                dict.set_item("http_errors", http_error_list)?;
                let assert_error_list = utils::create_assert_err_dict::create_assert_error_dict(py, &test_result.assert_errors)?;
                dict.set_item("assert_errors", assert_error_list)?;
                dict.set_item("timestamp", test_result.timestamp)?;
                Ok(dict.to_object(py))
            },
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Error: {:?}", e))),
        })
    })
}

#[pyclass]
struct StatusListenIter {}

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
        let should_stop = *core::status_share::SINGLE_SHOULD_STOP.lock();
        if should_stop {
            return Ok(None); // 停止迭代
        }
        let mut queue = core::status_share::SINGLE_RESULT_QUEUE.lock();
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
            dict.set_item("throughput_per_second_kb", test_result.throughput_per_second_kb)?;
            let http_error_list = utils::create_http_err_dict::create_http_error_dict(py, &test_result.http_errors)?;
            dict.set_item("http_errors", http_error_list)?;
            let assert_error_list = utils::create_assert_err_dict::create_assert_error_dict(py, &test_result.assert_errors)?;
            dict.set_item("assert_errors", assert_error_list)?;
            dict.set_item("timestamp", test_result.timestamp)?;
            Ok(Some(dict.to_object(py)))
        } else {
            Ok(Some(py.None())) // 暂时没有消息
        }
    }
}

#[pyfunction]
fn assert_option(py: Python, jsonpath: String, reference_object: PyObject) -> PyResult<PyObject>{
    let dict = PyDict::new(py);
    dict.set_item("jsonpath", jsonpath)?;
    dict.set_item("reference_object", reference_object)?;
    Ok(dict.to_object(py))
}

#[pyclass]
struct BatchListenIter {}

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
        let should_stop = *core::status_share::RESULTS_SHOULD_STOP.lock();
        if should_stop {
            return  Err(PyErr::new::<pyo3::exceptions::PyStopIteration, _>("No more data available"));
        }
        let mut queue = core::status_share::RESULTS_QUEUE.lock();
        if let Some(test_result) = queue.pop_front() {
            if test_result.total_data_kb == 0.0{
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
            Ok(Some(dict.to_object(py)))
        } else {
            Ok(Some(py.None()))
        }
    }
}


#[pyfunction]
#[pyo3(signature = (
test_duration_secs,
concurrent_requests,
api_endpoints,
verbose=false,
should_prevent=false,
))]
fn batch_async<'a>(
    py: Python<'a>,
    test_duration_secs: u64,
    concurrent_requests: usize,
    api_endpoints: &'a PyList,
    verbose: bool,
    should_prevent: bool,
) -> PyResult<&'a PyAny> {
    let endpoints = utils::parse_api_endpoints::new(py, api_endpoints)?;
    future_into_py(py, async move {
        let result = core::batch::batch(
           test_duration_secs,
            concurrent_requests,
            verbose,
            should_prevent,
            endpoints
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

#[pyfunction]
fn endpoint(py: Python,
            name: String,
            url: String,
            method: String,
            timeout_secs: u64,
            weight: u32,
            json: Option<PyObject>,
            headers: Option<PyObject>,
            cookies: Option<String>,
            assert_options: Option<&PyList>
) -> PyResult<PyObject>{
    let dict = PyDict::new(py);
    dict.set_item("name", name)?;
    dict.set_item("url", url)?;
    dict.set_item("method", method)?;
    dict.set_item("timeout_secs", timeout_secs)?;
    dict.set_item("weight", weight)?;
    if let Some(json) = json{
        dict.set_item("json", json)?;
    };
    if let Some(headers) = headers{
        dict.set_item("headers", headers)?;
    };
    if let Some(cookies) = cookies {
        dict.set_item("cookies", cookies)?;
    };
    if let Some(assert_options) = assert_options{
        dict.set_item("assert_options", assert_options)?;
    }
    Ok(dict.into())
}


#[pymodule]
#[pyo3(name = "atomic_bomb_engine")]
fn atomic_bomb_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<StatusListenIter>()?;
    m.add_class::<BatchListenIter>()?;
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(run_async, m)?)?;
    m.add_function(wrap_pyfunction!(batch_async, m)?)?;
    m.add_function(wrap_pyfunction!(assert_option, m)?)?;
    m.add_function(wrap_pyfunction!(endpoint, m)?)?;
    Ok(())
}
