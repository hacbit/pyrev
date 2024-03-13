def test(a, b: int) -> int:
    def aaa():
        return 1
    return len(bin(a + b + aaa())[2:])
def foo(a):
    print("this is foo")
    return a
b = lambda a: a + 1
[i + 1 for i in range(10)]
a = 1
foo(test(a))