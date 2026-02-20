def foo(*args):
    pass


def bar():
    # should resolve here
    def foo():
        pass

    a = foo()


b = bar()


@foo()
class Nate:
    def __init__(self, b, a: int, *args, **kwargs):
        pass

    def method(self, a: int) -> tuple[str,str]:
        pass
