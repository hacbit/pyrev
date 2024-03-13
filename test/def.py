def test(a, b: int) -> int:
    def aaa():
        return 1
    return len(bin(a + b + aaa())[2:])
def foo(a: int, b: int = 2, c = 3):
    print("this is foo")
    return a, b, c
b = lambda a: a + 1
[i + 1 for i in range(10)]
a = 1
foo(test(a))