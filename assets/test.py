# def foo(*args):
#     pass
#
#
# def bar():
#     # should resolve here
#     def foo():
#         pass
#
#     a = foo()
#
#
# b = bar()
#
#
# @foo()
# class Nate:
#     def __init__(self, b, a: int, *args, **kwargs):
#         pass
#
#     def method(self, a: int) -> tuple[str,str]:
#         pass

@workflows.workflow.define("failing-tool-call-workflow")
class FailingToolCallWorkflow:
    @workflows.workflow.entrypoint
    async def entrypoint(self) -> None:
        session = workflows_mistralai.RemoteSession()

        class WebSearchParams(BaseModel):
            query: str

        class WebSearchResult(BaseModel):
            result: str

        @workflows.activity(retry_policy_max_attempts=1)
        async def do_web_search(params: WebSearchParams) -> WebSearchResult:
            raise ValueError("This is a test error")

        agent = workflows_mistralai.Agent(
            model="mistral-medium-latest",
            description="Agent with web search tool",
            instructions="Follow the user instructions",
            name="web-search-agent",
            tools=[do_web_search],
        )
        logger.info("Workflow: Running agent")
        with contextlib.suppress(Exception):
            await workflows_mistralai.Runner.run(
                agent=agent,
                inputs="Call do_web_search tool with query 'What is the weather today?'",
                session=session,
            )

