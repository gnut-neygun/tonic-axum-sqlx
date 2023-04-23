import asyncio

from grpclib.client import Channel

from generated.python.helloworld import GreeterStub, HelloRequest


async def main():
    channel = Channel(host="localhost", port=50051)
    service = GreeterStub(channel)
    response = await service.say_hello(HelloRequest(name="world"))
    print(response)
    
    # don't forget to close the channel when done!
    channel.close()


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
