import pytest
import requests_mock


def add_token_request(requests_mocker):
    def match_authorization_and_body(request):
        """
        This will check if the request contains the expected values.
        """

        return request.headers["Authorization"] == "Bearer api_key" and request.json() == {}

    requests_mocker.post(
        "https://identity.cplane.cloud/token",
        additional_matcher=match_authorization_and_body,
        status_code=200,
        json={"token": "This is a token"},
    )


@pytest.fixture
def success_token_post():
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        yield


@pytest.fixture
def fail_token_post():
    with requests_mock.Mocker() as requests_mocker:
        requests_mocker.post("https://identity.cplane.cloud/token", status_code=400, json="")

        yield


@pytest.fixture
def fails_any_get():
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        requests_mocker.get(requests_mock.ANY, status_code=400, json="Some error")

        yield
