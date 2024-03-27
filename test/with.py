with open("./bbb", 'r') as f:
    content = f.read()

with open("aaaa", "rb") as f:
    f.write(content.encode())
