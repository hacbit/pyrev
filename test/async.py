import asyncio

class AIter:
    def __init__(self, ls: list):
        self.ls = ls
        self.index = 0

    def __aiter__(self):
        return self

    async def __anext__(self):
        if self.index < len(self.ls):
            i = self.ls[self.index]
            self.index += 1
            return i
        print("End of iteration")
        raise StopAsyncIteration

class ACtxMgr:
    def __init__(self, *args):
        pass

    async def __aenter__(self, *args):
        print("Entering")

    async def __aexit__(self, *args):
        print("Exiting")

async def foo():
    print("This is foo")
    return 42

async def bar():
    print(await foo())
    print("This is bar")
    return "bar end"

async def main():
    print(await bar())
    a = AIter([(1, 'a'), (2, 'b'), (3, 'c')])
    async for (idx, item) in a:
        print(idx, item)

    async with ACtxMgr() as a:
        print("In context manager")

    print("Main end")

asyncio.run(main())
