import asyncio
from proxybroker import Broker
import json


def writeFile(filename, content, mode='w+'):
    # 写文件内容
    try:
        fp = open(filename, mode)
        fp.write(content)
        fp.close()
        return True
    except Exception as e:
        return False

async def show(proxies):

    data = []

    while True:
        proxy = await proxies.get()
        if proxy is None:
            break

        tmp = {}
        tmp['host'] = proxy.host
        tmp['port'] = proxy.port

        data.append(tmp)
        # print(proxy.host)
        # print(proxy.port)

    d = json.dumps(data)
    print(d)
    writeFile('/tmp/tmp_proxy.json', d)

loop = asyncio.get_event_loop()

proxies = asyncio.Queue(loop=loop)
broker = Broker(proxies)
tasks = asyncio.gather(
    broker.find(types=['HTTPS'], limit=10),
    show(proxies))


loop.run_until_complete(tasks)
