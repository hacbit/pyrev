arr = [1, 3, 5, 7, 9]
for i, v in enumerate(arr):
    line = i + 1
    if line % 2 == 0:
        continue
    elif line == 3:
        break
    print(line, v)

print("end")
print("Test")
