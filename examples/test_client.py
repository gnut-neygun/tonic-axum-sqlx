import asyncio

from betterproto.lib.google.protobuf import Empty
from grpclib.client import Channel

from generated.python.object_api import ObjectApiStub


async def main():
    channel = Channel(host="localhost", port=3000)
    service = ObjectApiStub(channel)
    response = await service.list_objects(Empty())
    print(response)
    
    # don't forget to close the channel when done!
    channel.close()


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
