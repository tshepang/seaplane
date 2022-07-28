from typing import Generator

import pytest
from returns.result import Failure, Success

from seaplane.api.api_http import HTTPError
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
    assert token_api.access_token() == Success("This is a token")


def test_given_token_post_failure_call_returns_the_error_code(  # type: ignore
    token_api, fail_token_post
) -> None:
    assert token_api.access_token() == Failure(HTTPError(400))
