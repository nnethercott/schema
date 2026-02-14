from functools import cache
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

    @cache
    @foo
    @staticmethod
    def doubly_decorated(self):
        pass

    def another_method(self):
        def inner():
            pass

    @entrypoint
    def nate(self, args):
        pass

class Bar: ...
