from typing import Iterator, Optional
from _pyo3_runtime import PyAny


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
                assert_json_path:str| None = None) -> dict:
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
    :param should_prevent: 实验性功能！压测过程中是否阻止休眠，此参数为true时，需要使用管理员权限运行才有效果，使用此功能会增加电脑功耗，但在无人值守时会非常有用
    :param assert_json_path: 要提取值的jsonpath
    :return:
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
                assert_json_path:str| None = None) -> dict:
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
        :param should_prevent: 实验性功能！压测过程中是否阻止休眠，此参数为true时，需要使用管理员权限运行才有效果，使用此功能会增加电脑功耗，但在无人值守时会非常有用
        :param assert_json_path: 要提取值的jsonpath
        :return:
        """


class StatusListenIter:
    """
    实例化后返回一个监听器的生成器
    必须在压测的时候进行迭代，否则无法获取到数据
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
