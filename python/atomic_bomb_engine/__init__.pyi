from typing import Iterator, Optional, List, Dict, Any

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

def think_time_option(min_millis: int, max_millis: int) -> Dict[str, int]:
    """
    思考时间选项
    :param min_millis:
    :param max_millis:
    :return:
    """

def endpoint(
         name: str,
         url: str,
         method: str,
         weight: int,
         json: Dict | None = None,
         form_data: Dict | None = None,
         multipart_options: List[Dict]| None = None,
         headers: Dict | None = None,
         cookies: str | None = None,
         assert_options: List | None = None,
         think_time_option: Dict[str, int] | None = None,
         setup_options: List| None = None,
         ) -> Dict[str, Any]:
    """
    生成endpoint
    :param assert_options:
    :param form_data:
    :param name: 接口名称
    :param url: 接口地址
    :param method: 请求方法
    :param weight 权重
    :param json: 请求json
    :param form_data: 请求form表单
    :multipart_options: 附件
    :param headers: 请求头
    :param cookies: cookie
    :param assert_options: 断言参数
    :param think_time_option: 思考时间
    :param setup_options: 接口初始化选项
    """


def setup_option(
        name: str,
        url: str,
        method: str,
        json: Dict| None = None,
        form_data: Dict| None = None,
        multipart_options: List[Dict]| None = None,
        headers: Dict| None = None,
        cookies: str | None = None,
        jsonpath_extract: List| None = None) ->Dict[str, Any]:
    """
    初始化选项
    :param name: 接口名称
    :param url: 接口地址
    :param method: 请求方法
    :param json: 请求json
    :param form_data: 请求form表单
    :multipart_options: 附件
    :param headers: 请求头
    :param cookies: cookie
    :param jsonpath_extract: 通过jsonpath提取参数
    :return:
    """

def jsonpath_extract_option(key: str, jsonpath: str) -> Dict[str, str]:
    """
    jsonpath提取参数设置
    :param key: 全局key
    :param jsonpath: 提取jsonpath路径
    :return:
    """

def multipart_option(
        form_key: str,
        path: str,
        file_name: str,
        mime: str) -> Dict:
    """
    上传附件选项
    :param form_key: form表单的key，根据服务端选择，e.g: file， file1
    :param path: 文件路径
    :param file_name: 文件名
    :param mime: 文件类型，e.g: application/octet-stream,可以参考:https://developer.mozilla.org/zh-CN/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
    """

class BatchRunner:
    def __init__(self) -> None:
        ...

    def run(
            self,
             test_duration_secs: int,
             concurrent_requests: int,
             api_endpoints:List[Dict],
             step_option:Dict[str, int]|None=None,
             setup_options:List[Dict[str, Any]]|None=None,
             verbose:bool=False,
             should_prevent:bool=False,
             assert_channel_buffer_size:int=1024,
             timeout_secs=0,
             cookie_store_enable=True,
             ema_alpha: float=0,
    ) -> None:
        """
            批量压测
            :param test_duration_secs: 测试持续时间
            :param concurrent_requests: 并发数
            :param api_endpoints: 接口信息
            :param step_option: 阶梯加压选项
            :param setup_options: 初始化选项
            :param verbose: 打印详细信息
            :param should_prevent: 是否禁用睡眠
            :param assert_channel_buffer_size: 断言队列buffer大小
            :param timeout_secs: http超时时间
            :param cookie_store_enable: 是否为客户端启用持久性cookie存储。
            :param ema_alpha: 指数滑动平均参数，0-1之间,0为不使用，值越大曲线越平滑，但是越失真，建议使用0.1以下
        """
        ...

    def __iter__(self) -> 'BatchRunner':
        ...

    def __next__(self) -> Optional[Any]:
        ...