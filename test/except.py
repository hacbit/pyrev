a = 1
assert bool(a) == True, 'bool(a) == True'
try:
    assert bool(a) == False
except AssertionError as e:
    print('AssertionError: bool(a) == False')
finally:
    print('Done')