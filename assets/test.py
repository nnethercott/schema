from collections.abc import Callable
from typing import Any

def foo(arg: Any):
    def decorator(f: Callable):
        def _decorator(*args, **kwargs):
            print(arg)
            print(f(*args, **kwargs))
        return _decorator

    return decorator

@foo("nate")
def bar():
    pass

@foo(42)
def nate():
    print("hi")


@foo.bar.baz()
def jack():
    pass
