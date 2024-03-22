import atomic_bomb_engine
import os
import sys
import asyncio
import webbrowser
import time
import aiohttp
import aiosqlite
import json
from typing import Dict
from aiohttp import web
from atomic_bomb_engine import middleware


def ui(port: int = 8000, auto_open: bool = True):
    if port > 65535 or port < 0:
        raise ValueError(f"端口必须为0-65535")
    # 数据库连接
    db_connection = None

    async def get_db_connection():
        nonlocal db_connection
        if db_connection is None:
            db_connection = await aiosqlite.connect(":memory:")
            await db_connection.execute('CREATE TABLE results (id INTEGER PRIMARY KEY, data JSON)')
        return db_connection

    async def create_table():
        db = await get_db_connection()
        await db.commit()

    async def insert_result_data(data):
        db = await get_db_connection()
        json_data = json.dumps(data)
        await db.execute('INSERT INTO results (data) VALUES (?)', (json_data,))
        await db.commit()

    async def fetch_all_result_data():
        db = await get_db_connection()
        cursor = await db.execute('SELECT data FROM results ORDER BY id ASC')
        rows = await cursor.fetchall()
        results = [json.loads(row[0]) for row in rows]
        return results

    class Conn:
        """连接池对象"""

        def __init__(self, ws: aiohttp.web_ws.WebSocketResponse, heartbeat_time: float):
            self.ws = ws
            self.heartbeat_time = heartbeat_time

    # ws连接池
    connections: Dict[str, Conn] = dict()

    def decorator(func):
        async def start_service(*args, **kwargs):
            # 建表
            await create_table()

            # 定义ws接口
            async def websocket_handler(request):
                # 获取id
                if (client_id := request.match_info['id']) is None:
                    return web.Response(status=400, text="缺少id参数")

                ws = web.WebSocketResponse()
                await ws.prepare(request)
                # 将id加入连接池
                connections[client_id] = Conn(ws, time.time())

                # 心跳检测
                async def check_heartbeat():
                    while True:
                        await asyncio.sleep(0.3)
                        if time.time() - connections.get(client_id).heartbeat_time > 5:
                            sys.stderr.write(f"{time.ctime()}客户端{client_id} 未发送心跳，断开连接\n")
                            sys.stderr.flush()
                            connections.pop(client_id, None)
                            await ws.close()
                            break

                async def push_result():
                    result_iter = atomic_bomb_engine.BatchListenIter()
                    for item in result_iter:
                        if item:
                            await insert_result_data(item)
                            for cid, conn in list(connections.items()):
                                try:
                                    await conn.ws.send_json(item)
                                except ConnectionResetError:
                                    sys.stderr.write(f'{time.ctime()}-WebSocket ID {cid} 断开, 无法推送\n')
                                    sys.stderr.flush()
                                    # 从连接池中移除断开的连接
                                    connections.pop(cid, None)
                        await asyncio.sleep(0.2)

                # 推送任务
                push_task = asyncio.create_task(push_result())
                # 心跳任务
                check_heartbeat_task = asyncio.create_task(check_heartbeat())

                async for msg in ws:
                    if msg.type is web.WSMsgType.TEXT:
                        if msg.data.upper() == "PING":
                            # 更新心跳时间
                            connections[client_id].heartbeat_time = time.time()
                            await ws.send_str("PONG")
                    elif msg.type is web.WSMsgType.ERROR:
                        sys.stderr.write(f'WebSocket连接错误{ws.exception()}\n')
                        sys.stderr.flush()

                await push_task
                sys.stderr.write('WebSocket连接关闭\n')
                sys.stderr.flush()

                await check_heartbeat_task
                connections.pop(client_id, None)
                return ws

            # 定义run接口
            async def run_decorated_function(request):
                result = await func(*args, **kwargs)
                return web.json_response(result)

            # 定义history接口
            async def history(request):
                results = await fetch_all_result_data()
                return web.json_response(results)

            # 重定向到首页
            async def redirect_to_index(request):
                raise web.HTTPFound('/static/index.html')

            app = web.Application(middlewares=[middleware.cors_middleware])
            # 静态页面
            app.router.add_static('/static', path=os.path.join(os.path.dirname(__file__), 'dist'), name='dist')
            # 路由
            app.add_routes([web.get('/', redirect_to_index),
                            web.get('/ws/{id}', websocket_handler),
                            web.get('/run', run_decorated_function),
                            web.get('/history', history),
                            ])
            runner = web.AppRunner(app)
            await runner.setup()
            site = web.TCPSite(runner, '0.0.0.0', port)
            await site.start()
            # 等待协程运行完成
            await asyncio.Event().wait()

        sys.stderr.write(f"服务启动成功: http://localhost:{port}\n")
        sys.stderr.flush()
        if auto_open:
            webbrowser.open(f"http://localhost:{port}")
        return start_service

    return decorator
