@workflows.workflow.define
class Foo():
    def __init__(self):
        self.foo = "bar"

    @workflows.activity(name="nate")
    def bar(self):
        pass
