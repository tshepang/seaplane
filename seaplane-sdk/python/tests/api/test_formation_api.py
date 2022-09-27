from typing import Any, Generator

import pytest
import requests_mock
from returns.result import Success

from seaplane.api.formation_api import FormationAPI
from seaplane.configuration import Configuration
from seaplane.model.compute.formation_metadata import FormationMetadata

from ..conftest import add_token_request, fails_any_get  # noqa: F401


@pytest.fixture
def create_formation_post() -> Generator[None, None, None]:
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        def match_authorization(request: Any) -> Any:
            return (
                request.headers["Authorization"]
                == "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSJ9.eyJpYXQiOjE2NjM2MTk5MTUsIm5iZiI6MTY2MzYxOTkxNSwiZXhwIjoxNjYzNjE5OTc1LCJpc3MiOiJpZGVudGl0eS5jcGxhbmUuY2xvdWQiLCJhdWQiOiJjcGxhbmUuY2xvdWQiLCJzdWIiOiI0MDEiLCJ0ZW5hbnQiOiI0MDEiLCJzdWJkb21haW4iOiJ0b25pLXRlc3RzIn0.CgSeHIa2fOq0Ro68ALXLkBgNQhXVOMUFy5cUG-R7bVWwtAblhqO6T0PbOzsmRXemTXph94QBSXWoqpPSj079CQ"  # noqa
                and request.query == "active=false"
            )

        requests_mocker.post(
            "https://compute.cplane.cloud/v1/formations/test-formation",
            additional_matcher=match_authorization,
            status_code=201,
            json=[],
        )

        yield


@pytest.fixture
def create_formation_post_with_query_params() -> Generator[None, None, None]:
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        def match_authorization(request: Any) -> Any:
            return request.query == "active=true&source=any_source"

        requests_mocker.post(
            "https://compute.cplane.cloud/v1/formations/test-formation",
            additional_matcher=match_authorization,
            status_code=201,
            json=[],
        )

        yield


@pytest.fixture
def get_all_formations() -> Generator[None, None, None]:
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        def match_authorization(request: Any) -> Any:
            return (
                request.headers["Authorization"]
                == "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSJ9.eyJpYXQiOjE2NjM2MTk5MTUsIm5iZiI6MTY2MzYxOTkxNSwiZXhwIjoxNjYzNjE5OTc1LCJpc3MiOiJpZGVudGl0eS5jcGxhbmUuY2xvdWQiLCJhdWQiOiJjcGxhbmUuY2xvdWQiLCJzdWIiOiI0MDEiLCJ0ZW5hbnQiOiI0MDEiLCJzdWJkb21haW4iOiJ0b25pLXRlc3RzIn0.CgSeHIa2fOq0Ro68ALXLkBgNQhXVOMUFy5cUG-R7bVWwtAblhqO6T0PbOzsmRXemTXph94QBSXWoqpPSj079CQ"  # noqa
            )

        requests_mocker.get(
            "https://compute.cplane.cloud/v1/formations",
            additional_matcher=match_authorization,
            status_code=200,
            json=["formation-example", "test-formation"],
        )

        yield


@pytest.fixture
def get_metadata() -> Generator[None, None, None]:
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        def match_authorization(request: Any) -> Any:
            return (
                request.headers["Authorization"]
                == "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSJ9.eyJpYXQiOjE2NjM2MTk5MTUsIm5iZiI6MTY2MzYxOTkxNSwiZXhwIjoxNjYzNjE5OTc1LCJpc3MiOiJpZGVudGl0eS5jcGxhbmUuY2xvdWQiLCJhdWQiOiJjcGxhbmUuY2xvdWQiLCJzdWIiOiI0MDEiLCJ0ZW5hbnQiOiI0MDEiLCJzdWJkb21haW4iOiJ0b25pLXRlc3RzIn0.CgSeHIa2fOq0Ro68ALXLkBgNQhXVOMUFy5cUG-R7bVWwtAblhqO6T0PbOzsmRXemTXph94QBSXWoqpPSj079CQ"  # noqa
            )

        requests_mocker.get(
            "https://compute.cplane.cloud/v1/formations/test-formation",
            additional_matcher=match_authorization,
            status_code=200,
            json={"url": "https://example.url"},
        )

        yield


@pytest.fixture
def already_created_formation() -> Generator[None, None, None]:
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        requests_mocker.post(
            "https://compute.cplane.cloud/v1/formations/test-formation",
            status_code=409,
            json="There is already a formation with that name",
        )

        yield


@pytest.fixture
def delete_formation() -> Generator[None, None, None]:
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        requests_mocker.delete(
            "https://compute.cplane.cloud/v1/formations/test-formation",
            status_code=200,
            json=[],
        )

        yield


@pytest.fixture
def formation_api() -> Generator[FormationAPI, None, None]:
    config = Configuration()
    config.set_api_key("api_key")
    formation_api = FormationAPI(config)

    yield formation_api


def test_given_formation_name_create_a_new_formation(  # type: ignore
    formation_api, create_formation_post
) -> None:
    assert formation_api.create("test-formation") == Success([])


def test_given_create_formation_already_created_returns_409_http_error(  # type: ignore
    formation_api, already_created_formation
) -> None:
    failure = formation_api.create("test-formation").failure()
    assert failure.status == 409
    assert failure.message == "There is already a formation with that name"


def test_given_get_all_api_call_returns_two_formations(  # type: ignore
    formation_api, get_all_formations
) -> None:
    assert formation_api.get_all() == Success(["formation-example", "test-formation"])


def test_given_get_all_api_call_returns_400_http_error(  # type: ignore
    formation_api, fails_any_get  # noqa: F811
) -> None:
    failure = formation_api.get_all().failure()
    assert failure.status == 400
    assert failure.message == "Some error"


def test_given_get_all_api_call_includes_active_and_source_query_params(  # type: ignore
    formation_api, create_formation_post_with_query_params
) -> None:
    assert formation_api.create("test-formation", active=True, source="any_source") == Success([])


def test_given_get_metadata_call_parses_the_response_correctly(  # type: ignore
    formation_api, get_metadata
) -> None:
    assert formation_api.get_metadata("test-formation") == Success(
        FormationMetadata(url="https://example.url")
    )


def test_given_remove_formation_call_returns(  # type: ignore
    formation_api, delete_formation
) -> None:
    assert formation_api.delete("test-formation") == Success([])
