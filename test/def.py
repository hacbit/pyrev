def test(a: int, b) -> int:
    def aaa():
        return 1
    return len(bin(a + b + aaa())[2:])
def foo(a):
    print("this is foo")
    return a
b = lambda a: a
[i + 1 for i in range(10)]
a = 1
foo(test(a))