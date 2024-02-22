arr = [1, 3, 5, 7, 9]
for i, v in enumerate(arr):
    line = i + 1
    print(line, v)
    if i == 2:
        break
    else:
        continue
else:
    print("end")
print("Test")