def foo():
    pass

def bar():
    # should resolve here
    def foo():
        pass

