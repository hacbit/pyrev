a = 15
if a < 100:
    if not a >= 10:
        print("a < 10")
    elif a == 10:
        print("a == 10")
    elif a < 50 and a > 20:
        print("20 < a < 50")
    else:
        print("10 < a <= 20")
        print("or a >= 50")
elif a == 100:
    print("a == 100")
else:
    print("a > 100")
print(a)
