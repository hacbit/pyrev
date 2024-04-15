from base64 import *
flag = input()
dec = b64decode(flag.encode())
print("base64 decode: ")
print(dec)
