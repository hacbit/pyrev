a = 1
assert bool(a) == True, 'bool(a) == True'
try:
    #assert bool(a) == False
    a = 1 + 'a'
except AssertionError:
    print('AssertionError: bool(a) == False')
except TypeError:
    print('TypeError: a = 1 + "a"')
except Exception as e:
    print(f'Unexpected error: {e}')
finally:
    print('Done')
