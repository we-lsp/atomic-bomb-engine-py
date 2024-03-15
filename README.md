# atomic-bomb-engine-py
#### [atomic-bomb-engine](https://github.com/qyzhg/atomic-bomb-engine)的python包装实现

<img src="atomic-bomb-engine-logo.png" width="350px" height="350px" alt="logo">


## 前端仓库
#### [atomic-bomb-engine-front](https://github.com/GiantAxeWhy/atomic-bomb-engine-front)

## 使用条件：
- python版本 >= 3.8
- windows(x86), linux(x86), mac

## 使用方法：
- ### 准备开始
通过pip安装 （0.5.0版本之前）
```shell
pip install atomic-bomb-engine-py
```
在python中引用时注意，需要引用atomic_bomb_engine, 而不是atomic_bomb_engine_py
<br/> 为了避免混淆，0.5.0版本之后，pip更换了包名，更改为atomic-bomb-engine，
```shell
pip install atomic-bomb-engine
```
在python中导入
```python
import atomic_bomb_engine
```
异步使用的时候，还需要引用asyncio
```python
import asyncio
```
- ### 开始压测
  - ~~单接口压测~~ 
  <br/>⚠️由于和批量压测功能重叠，单接口压测将在下个版本中删除
  
  单接口压测可以使用run_async方法
  函数签名和解释如下
  ```python
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
        assert_options: List[Dict[str, Any]] | None
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
            :param should_prevent: 实验性功能！压测过程中是否阻止休眠，此参数为true时，需要使用管理员权限运行才有效果，使用此功能会增加电脑功耗，但在无人值守时会非常有用
            :param assert_options: 断言，传入一个字典列表，key必须包含两个：jsonpath和reference_object e.g. [{"jsonpath": "$.code", "reference_object": 429}, {"jsonpath": "$.code", "reference_object": "300"}]， 也可以使用本包中的assert_option方法生成option
            :return: Dict
            """
  ```
  使用assert_options时，要传入一个字典，但是如果感觉这个字典比较难以记忆的话，可以使用本包中的assert_option方法返回这个字典
    ```python
      async def run():
          print("开始压测")
          result = await atomic_bomb_engine.run_async(
            url="https://xxxxx.xxx",
            method="GET",
            test_duration_secs=60,
            concurrent_requests=200,
            timeout_secs=10,
            verbose=False,
            should_prevent=True,
            assert_options=[
                atomic_bomb_engine.assert_option("$.code", 429),
                atomic_bomb_engine.assert_option("$.code", 200)
            ])
          print(result)
    ```
    jsonpath如果不会用的话，建议去[jsonpath](https://jsonpath.com/)学习
  - 单接口压测结果实时监听
  可以迭代包中的StatusListenIter类进行压测结果的监听
  ```python
  async def listen():
    iterator = atomic_bomb_engine.StatusListenIter()
    for message in iterator:
        if message:
            print(message)
        else:
            await asyncio.sleep(0.3)
  ```
   在这个循环中，你可以做落库等各种操作，不再赘述
  
    - 压测时同时监听可以这样使用
  ```python
  async def main():
    await asyncio.gather(
        run(),
        listen(),
    )
  
  
  if __name__ == "__main__":
    asyncio.run(main())
  ```

  - 多接口压测

多接口压测可以使用batch_async方法进行操作，函数签名和解释如下
 ```python
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
 ```
同样的本包中也包含了一个对api_endpoint的包装：endpoint方法，方便调用，endpoint中的assert_options中也可以套用assert_option方法
 ```python
    async def run_batch():
        result = await atomic_bomb_engine.batch_async(
            test_duration_secs=10,
            concurrent_requests=10,
            api_endpoints=[
                atomic_bomb_engine.endpoint(
                    name="test1",
                    url="https:xxxxx1.xx",
                    method="get",
                    weight=1,
                    timeout_secs=10,
                    assert_options=[atomic_bomb_engine.assert_option(jsonpath="$.code", reference_object=200)]
                ),
                atomic_bomb_engine.endpoint(
                    name="test2",
                    url="https://xxxxx2.xx",
                    method="get",
                    weight=1,
                    timeout_secs=10)
            ])
        print(result)
 ```
    
监听时可以使用BatchListenIter生成器
```python
async def listen_batch():
    iterator = atomic_bomb_engine.BatchListenIter()
    for message in iterator:
        if message:
            print(message)
        else:
            await asyncio.sleep(0.3)
```
同时调用时同单接口
```python 
async def main():
    await asyncio.gather(
        run_batch(),
        listen_batch(),
    )


if __name__ == "__main__":
    asyncio.run(main())
```

# 压测时使用ui界面监控

0.5.0版本后，添加了ui页面，支持批量压测方法
<br/>导入
```python
from atomic_bomb_engine import server
```
使用
```python
import asyncio

import atomic_bomb_engine
from atomic_bomb_engine import server


@server.ui(port=8000)
async def run_batch():
    result = await atomic_bomb_engine.batch_async(
        test_duration_secs=120,
        concurrent_requests=100,
        verbose=False,
        api_endpoints=[
            atomic_bomb_engine.endpoint(name="test-baidu",url="https://baidu.com",method="GET",weight=1,timeout_secs=10),
            atomic_bomb_engine.endpoint(name="test-google", url="https://google.com", method="GET", weight=1, timeout_secs=10),
        ])
    print(result)
    return result


if __name__ == '__main__':
    asyncio.run(run_batch())
```

使用server.ui装饰器，可以给批量压测方法启动一个简单的web服务器，不需要再手动监听BatchListenIter生成器

## bug和需求
- 如果发现了bug，把复现步骤一起写到Issus中哈
- 如果有需求也可以在Issues中讨论
- 本程序是本人业余时间开发，不太准备保证时效性，但是如果有时间，一定第一时间回复和修改bug

## TODO
 - ~~前端展示页面~~ done
 - 接口关联
