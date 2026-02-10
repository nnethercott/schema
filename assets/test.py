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

@workflows.workflow.define
class Nate(Bar, Baz):
    def __init__(self):
        pass
    @entrypoint
    def entrypoint(self, args):
        pass
