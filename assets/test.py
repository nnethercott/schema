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
    @cache
    @foo
    @staticmethod
    def doubly_decorated(self):
        pass

    def a_method(self):
        def inner():
            pass

    @workflows.activity()
    def activity_in_class():
        pass

    @entrypoint
    def nate(self, args):
        @workflows.activity()
        def activity_in_wrapper():
            pass

class Bar: ...
