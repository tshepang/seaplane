from typing import Any, Generator

import pytest
import requests_mock


def add_token_request(requests_mocker: Any) -> None:
    def match_authorization_and_body(request: Any) -> Any:
        """
        This will check if the request contains the expected values.
        """

        return request.headers["Authorization"] == "Bearer api_key" and request.json() == {}

    requests_mocker.post(
        "https://identity.cplane.cloud/token",
        additional_matcher=match_authorization_and_body,
        status_code=200,
        json={
            "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSJ9.eyJpYXQiOjE2NjM2MTk5MTUsIm5iZiI6MTY2MzYxOTkxNSwiZXhwIjoxNjYzNjE5OTc1LCJpc3MiOiJpZGVudGl0eS5jcGxhbmUuY2xvdWQiLCJhdWQiOiJjcGxhbmUuY2xvdWQiLCJzdWIiOiI0MDEiLCJ0ZW5hbnQiOiI0MDEiLCJzdWJkb21haW4iOiJ0b25pLXRlc3RzIn0.CgSeHIa2fOq0Ro68ALXLkBgNQhXVOMUFy5cUG-R7bVWwtAblhqO6T0PbOzsmRXemTXph94QBSXWoqpPSj079CQ"  # noqa
        },
    )


@pytest.fixture
def success_token_post() -> Generator[None, None, None]:
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        yield


@pytest.fixture
def fail_token_post() -> Generator[None, None, None]:
    with requests_mock.Mocker() as requests_mocker:
        requests_mocker.post("https://identity.cplane.cloud/token", status_code=400, json="")

        yield


@pytest.fixture
def fails_any_get() -> Generator[None, None, None]:
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        requests_mocker.get(requests_mock.ANY, status_code=400, json="Some error")

        yield
