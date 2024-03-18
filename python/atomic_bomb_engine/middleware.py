async def cors_middleware(app, handler):
    async def cors_handler(request):
        response = await handler(request)
        response.headers['Access-Control-Allow-Origin'] = '*'
        response.headers['Access-Control-Allow-Methods'] = 'POST, GET, OPTIONS'
        response.headers['Access-Control-Allow-Headers'] = 'X-Requested-With, Content-Type'
        return response
    return cors_handler