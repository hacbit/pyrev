a = 1
assert bool(a) == True, 'bool(a) == True'
try:
    #assert bool(a) == False
    a = 1 + 'a'
except AssertionError as e:
    print('AssertionError: bool(a) == False')
except TypeError as e:
    print('TypeError: a = 1 + "a"')
except:
    print('Unexpected error')
finally:
    print('Done')