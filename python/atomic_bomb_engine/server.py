import atomic_bomb_engine
import os
from aiohttp import web
import asyncio


def ui(func):
    async def start_service(*args, **kwargs):
        async def run_decorated_function(request):
            result = await func(*args, **kwargs)
            return web.json_response(result)

        async def redirect_to_index(request):
            raise web.HTTPFound('/static/index.html')

        app = web.Application()
        app.router.add_static('/static', path=os.path.join(os.path.dirname(__file__), 'dist'), name='dist')

        app.add_routes([web.get('/', redirect_to_index),
                        web.get('/ws', websocket_handler),
                        web.get('/run', run_decorated_function)])

        runner = web.AppRunner(app)
        await runner.setup()
        site = web.TCPSite(runner, '0.0.0.0', 8000)
        await site.start()

        event = asyncio.Event()
        await event.wait()
    print("服务启动成功: http://localhost:8000")
    return start_service


async def websocket_handler(request):
    ws = web.WebSocketResponse()
    await ws.prepare(request)

    async def push_result():
        result_iter = atomic_bomb_engine.BatchListenIter()
        for item in result_iter:
            if item:
                await ws.send_json(item)
            await asyncio.sleep(0.3)

    push_task = asyncio.create_task(push_result())

    async for msg in ws:
        if msg.type is web.WSMsgType.TEXT:
            if msg.data.upper() == "PING":
                await ws.send_str("PONG")
        elif msg.type is web.WSMsgType.ERROR:
            print(f'WebSocket连接错误{ws.exception()}')

    await push_task
    print('WebSocket连接关闭')
    return ws
