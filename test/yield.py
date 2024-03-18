def get_gen():
    yield g.gi_frame.f_back

g = get_gen()
(lambda: (yield))()
lambda x: (yield x + 1)
