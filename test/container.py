a = [1, "hello", 114.514]
b = (1, a, "world")
c = {1: 1, "b": b'bb', "c": "ccc", "p": print}
key = 111
d = {key: 0x61, 'b': None}
e = {1232, 0, None, print}
c.get('p')(a, b, c, d)