#!/usr/bin/env python3

import asyncio
import websockets
import requests

ENDPOINT = "://localhost:3030"


async def echo_test():
    async with websockets.connect("ws" + ENDPOINT + "/echo") as websocket:
        await websocket.send("from python")
        print(await websocket.recv() == "from python")

def index_test():
    r = requests.get("http" + ENDPOINT)
    print(r.text)


asyncio.run(echo_test())
index_test()