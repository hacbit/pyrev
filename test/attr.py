a = 1234
b = a.to_bytes(2, 'big')
print(b)
print('\x61\x62\x63\x64'.encode('utf-8').hex())
print(a.__class__.__base__)