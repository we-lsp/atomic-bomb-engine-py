from typing import Iterator, Optional
from _pyo3_runtime import PyAny
from typing import List, Dict, Any

def assert_option(jsonpath: str, reference_object: any) -> Dict[str, Any]:
    """
    生成assert option
    :param jsonpath: jsonpath取值地址
    :param reference_object: 断言的值
    """


def step_option(increase_step: int, increase_interval: int) -> Dict[str, int]:
    """
    生成step option
    :param increase_step: 阶梯步长
    :param increase_interval: 阶梯间隔
    """

def endpoint(
         name: str,
         url: str,
         method: str,
         timeout_secs: int,
         weight: int,
         json: Dict| None = None,
         form_data: Dict| None = None,
         headers: Dict| None = None,
         cookies: str | None = None,
         assert_options: List| None = None
         ) -> Dict[str, Any]:
    """
    生成endpoint
    :param assert_options:
    :param form_data:
    :param name: 接口名称
    :param url: 接口地址
    :param method: 请求方法
    :param timeout_secs: 超时时间(秒)
    :param weight 权重
    :param json: 请求json
    :form_data: 请求form表单
    :param headers: 请求头
    :param cookies: cookie
    :assert_options: 断言参数
    """


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
             step_option:Dict[str, int]=None,
             verbose:bool=False,
             should_prevent:bool=False) ->Dict:
    """
        批量压测
        :param test_duration_secs: 测试持续时间
        :param concurrent_requests: 并发数
        :param api_endpoints: 接口信息
        :param step_option: 阶梯加压选项
        :param verbose: 打印详细信息
        :param should_prevent: 是否禁用睡眠
    """
