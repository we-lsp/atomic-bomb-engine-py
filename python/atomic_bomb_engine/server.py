from flask import Flask
import asyncio
import atomic_bomb_engine

def ui(f):
    def wrapper(*args, **kwargs):
        app = Flask(__name__)
        res = ""
        async def listen_batch():
            iterator = atomic_bomb_engine.BatchListenIter()
            for message in iterator:
                if message:
                    print(message)
                else:
                    await asyncio.sleep(0.3)

        async def start():
            nonlocal res
            result = await asyncio.gather(f(), listen_batch())
            res = result[0]

        @app.route("/run")
        def run():
            asyncio.run(start())
            return res

        app.run()
        return f(*args, **kwargs)
    return wrapper
