from typing import Generator

import pytest
from returns.result import Success

from seaplane.api.token_api import TokenAPI
from seaplane.configuration import Configuration


@pytest.fixture
def token_api() -> Generator[TokenAPI, None, None]:
    config = Configuration()
    config.set_api_key("api_key")
    token_api = TokenAPI(config)

    yield token_api


def test_given_token_post_call_returns_the_access_token(  # type: ignore
    token_api, success_token_post
) -> None:
    assert token_api._request_access_token() == Success(
        "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSJ9.eyJpYXQiOjE2NjM2MTk5MTUsIm5iZiI6MTY2MzYxOTkxNSwiZXhwIjoxNjYzNjE5OTc1LCJpc3MiOiJpZGVudGl0eS5jcGxhbmUuY2xvdWQiLCJhdWQiOiJjcGxhbmUuY2xvdWQiLCJzdWIiOiI0MDEiLCJ0ZW5hbnQiOiI0MDEiLCJzdWJkb21haW4iOiJ0b25pLXRlc3RzIn0.CgSeHIa2fOq0Ro68ALXLkBgNQhXVOMUFy5cUG-R7bVWwtAblhqO6T0PbOzsmRXemTXph94QBSXWoqpPSj079CQ"  # noqa
    )
    assert (
        token_api.get_token()
        == "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSJ9.eyJpYXQiOjE2NjM2MTk5MTUsIm5iZiI6MTY2MzYxOTkxNSwiZXhwIjoxNjYzNjE5OTc1LCJpc3MiOiJpZGVudGl0eS5jcGxhbmUuY2xvdWQiLCJhdWQiOiJjcGxhbmUuY2xvdWQiLCJzdWIiOiI0MDEiLCJ0ZW5hbnQiOiI0MDEiLCJzdWJkb21haW4iOiJ0b25pLXRlc3RzIn0.CgSeHIa2fOq0Ro68ALXLkBgNQhXVOMUFy5cUG-R7bVWwtAblhqO6T0PbOzsmRXemTXph94QBSXWoqpPSj079CQ"  # noqa
    )


def test_given_token_post_failure_call_returns_the_error_code(  # type: ignore
    token_api, fail_token_post
) -> None:
    failure = token_api._request_access_token().failure()

    assert failure.status == 400
    assert failure.message == ""
