import pytest
import requests
import requests_mock
from returns.result import Failure, Success

from seaplane import Configuration
from seaplane.api import HTTPError, TokenAPI


@pytest.fixture
def token_api():
    config = Configuration()
    config.set_api_key("api_key")
    token_api = TokenAPI(config)

    yield token_api


def test_given_token_post_call_returns_the_access_token(token_api, success_token_post) -> None:
    assert token_api.access_token() == Success("This is a token")


def test_given_token_post_failure_call_returns_the_error_code(token_api, fail_token_post) -> None:
    assert token_api.access_token() == Failure(HTTPError(400))
