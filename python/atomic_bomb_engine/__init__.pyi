from typing import Iterator, Optional
from _pyo3_runtime import PyAny
from typing import List, Dict, Any

def assert_option(jsonpath: str, reference_object: any) -> Dict[str, Any]:
    """
    生成assert option
    :param jsonpath: jsonpath取值地址
    :param reference_object: 断言的值
    """

def endpoint(
         name: str,
         url: str,
         method: str,
         timeout_secs: int,
         weight: int,
         json: Dict| None = None,
         headers: Dict| None = None,
         cookies: str | None = None,
         assert_options: List| None = None
         ) -> Dict[str, Any]:
    """
    生成endpoint
    :param name: 接口名称
    :param url: 接口地址
    :param method: 请求方法
    :param timeout_secs: 超时时间(秒)
    :param weight 权重
    :param json: 请求json
    :param headers: 请求头
    :param cookies: cookie
    :assert_options: 断言参数
    """

def run(
        url: str,
        method: str,
        test_duration_secs: int,
        concurrent_requests: int,
        timeout_secs: int,
        verbose: bool = False,
        json_str: str | None = None,
        form_data_str: str | None = None,
        headers: list[str] | None = None,
        cookie: str | None = None,
        should_prevent:bool = False,
        assert_options: List[Dict[str, Any]] | None
) -> dict:
    """
    同步启动压测引擎
    :param url: 压测地址
    :param method: 请求方式
    :param test_duration_secs: 持续时间
    :param concurrent_requests: 并发量
    :param timeout_secs: 接口超时时间
    :param verbose: 开启详情日志
    :param json_str: 使用json请求发送请求,使用json字符串,不要使用字典类型
    :param form_data_str: 使用form方式发送请求
    :param headers: 添加请求头
    :param cookie: 添加cookie
    :param should_prevent: 实验性功能!压测过程中是否阻止休眠,此参数为true时,需要使用管理员权限运行才有效果,使用此功能会增加电脑功耗,但在无人值守时会非常有用
    :param assert_options: 断言,传入一个字典列表,key必须包含两个:jsonpath和reference_object e.g. [{"jsonpath": "$.code", "reference_object": 429}, {"jsonpath": "$.code", "reference_object": "300"}]， 也可以使用本包中的assert_option方法生成option
    :return: Dict
    """

async def run_async(
        url: str,
        method: str,
        test_duration_secs: int,
        concurrent_requests: int,
        timeout_secs: int,
        verbose: bool = False,
        json_str: str | None = None,
        form_data_str: str | None = None,
        headers: str | None = None,
        cookie: str | None = None,
        should_prevent:bool = False,
        assert_options: List[Dict[str, Any]] | None = None
) -> dict:
    """
    异步启动压测引擎
    :param url: 压测地址
    :param method: 请求方式
    :param test_duration_secs: 持续时间
    :param concurrent_requests: 并发量
    :param timeout_secs: 接口超时时间
    :param verbose: 开启详情日志
    :param json_str: 使用json请求发送请求,使用json字符串,不要使用字典类型
    :param form_data_str: 使用form方式发送请求
    :param headers: 添加请求头
    :param cookie: 添加cookie
    :param should_prevent: 实验性功能!压测过程中是否阻止休眠,此参数为true时,需要使用管理员权限运行才有效果,使用此功能会增加电脑功耗,但在无人值守时会非常有用
    :param assert_options: 断言,传入一个字典列表,key必须包含两个:jsonpath和reference_object e.g. [{"jsonpath": "$.code", "reference_object": 429}, {"jsonpath": "$.code", "reference_object": "300"}]， 也可以使用本包中的assert_option方法生成option
    :return: Dict
    """


class StatusListenIter:
    """
    实例化后返回一个监听器的生成器
    必须在单接口压测的时候进行迭代，否则无法获取到数据
    建议如果没有获取到数据的的时候，添加一个sleep，不需要太密集的查询，引擎的生产速度是1秒一次
    e.g.
        async def listen():
            for message in performance_engine.StatusListenIter():
                if message:
                    # 在这里处理业务逻辑，可以落库或者推送ws的操作
                    print(message)
            await asyncio.sleep(0.3)
    """
    def __iter__(self) -> "StatusListenIter": ...
    def __next__(self) -> Optional[PyAny]: ...


class BatchListenIter:
    """
    实例化后返回一个监听器的生成器
    必须在批量压测的时候进行迭代，否则无法获取到数据
    建议如果没有获取到数据的的时候，添加一个sleep，不需要太密集的查询，引擎的生产速度是1秒一次
    e.g.
        async def listen():
            for message in performance_engine.BatchListenIter():
                if message:
                    # 在这里处理业务逻辑，可以落库或者推送ws的操作
                    print(message)
            await asyncio.sleep(0.3)
    """
    def __iter__(self) -> "BatchListenIter": ...
    def __next__(self) -> Optional[PyAny]: ...


async def batch_async(
             test_duration_secs: int,
             concurrent_requests: int,
             api_endpoints:List[Dict],
             verbose:bool=False,
             should_prevent:bool=False) ->Dict:
    """
        批量压测
        :param test_duration_secs: 测试持续时间
        :param concurrent_requests: 并发数
        :param api_endpoints: 接口信息
        :param verbose: 打印详细信息
        :param should_prevent: 是否禁用睡眠
    """
