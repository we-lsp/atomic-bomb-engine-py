import atomic_bomb_engine
import os
import sys
from aiohttp import web
import asyncio
from atomic_bomb_engine import middleware


def ui(port: int=8000):
    if port > 65535 or port < 0:
        raise ValueError(f"端口必须为0-65535")
    def decorator(func):
        async def start_service(*args, **kwargs):
            # 定义ws接口
            async def websocket_handler(request):
                ws = web.WebSocketResponse()
                await ws.prepare(request)

                async def push_result():
                    result_iter = atomic_bomb_engine.BatchListenIter()
                    for item in result_iter:
                        if item:
                            await ws.send_json(item)
                        await asyncio.sleep(0.2)

                push_task = asyncio.create_task(push_result())

                async for msg in ws:
                    if msg.type is web.WSMsgType.TEXT:
                        if msg.data.upper() == "PING":
                            await ws.send_str("PONG")
                    elif msg.type is web.WSMsgType.ERROR:
                        sys.stderr.write(f'WebSocket连接错误{ws.exception()}\n')
                        sys.stderr.flush()

                await push_task
                sys.stderr.write('WebSocket连接关闭\n')
                sys.stderr.flush()
                return ws
            # 定义run接口
            async def run_decorated_function(request):
                result = await func(*args, **kwargs)
                return web.json_response(result)
            # 重定向到首页
            async def redirect_to_index(request):
                raise web.HTTPFound('/static/index.html')

            app = web.Application(middlewares=[middleware.cors_middleware])
            # 静态页面
            app.router.add_static('/static', path=os.path.join(os.path.dirname(__file__), 'dist'), name='dist')
            # 路由
            app.add_routes([web.get('/', redirect_to_index),
                            web.get('/ws', websocket_handler),
                            web.get('/run', run_decorated_function)])
            runner = web.AppRunner(app)
            await runner.setup()
            site = web.TCPSite(runner, '0.0.0.0', port)
            await site.start()
            # 等待协程运行完成
            await asyncio.Event().wait()
        sys.stderr.write(f"服务启动成功: http://localhost:{port}\n")
        sys.stderr.flush()
        return start_service
    return decorator
