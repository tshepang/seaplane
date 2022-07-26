import pytest
import requests_mock
from returns.result import Failure, Success

from seaplane import Configuration
from seaplane.api import FormationAPI, HTTPError
from seaplane.model import FormationMetadata

from ..conftest import add_token_request, fails_any_get  # noqa: F401


@pytest.fixture
def create_formation_post():
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        def match_authorization(request):
            return (
                request.headers["Authorization"] == "Bearer This is a token"
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
def create_formation_post_with_query_params():
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        def match_authorization(request):
            return request.query == "active=true&source=any_source"

        requests_mocker.post(
            "https://compute.cplane.cloud/v1/formations/test-formation",
            additional_matcher=match_authorization,
            status_code=201,
            json=[],
        )

        yield


@pytest.fixture
def get_all_formations():
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        def match_authorization(request):
            return request.headers["Authorization"] == "Bearer This is a token"

        requests_mocker.get(
            "https://compute.cplane.cloud/v1/formations",
            additional_matcher=match_authorization,
            status_code=200,
            json=["formation-example", "test-formation"],
        )

        yield


@pytest.fixture
def get_metadata():
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        def match_authorization(request):
            return request.headers["Authorization"] == "Bearer This is a token"

        requests_mocker.get(
            "https://compute.cplane.cloud/v1/formations/test-formation",
            additional_matcher=match_authorization,
            status_code=200,
            json={"url": "https://example.url"},
        )

        yield


@pytest.fixture
def already_created_formation():
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        requests_mocker.post(
            "https://compute.cplane.cloud/v1/formations/test-formation",
            status_code=409,
            json="There is already a formation with that name",
        )

        yield


@pytest.fixture
def delete_formation():
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        requests_mocker.delete(
            "https://compute.cplane.cloud/v1/formations/test-formation",
            status_code=200,
            json=[],
        )

        yield


@pytest.fixture
def formation_api():
    config = Configuration()
    config.set_api_key("api_key")
    formation_api = FormationAPI(config)

    yield formation_api


def test_given_formation_name_create_a_new_formation(formation_api, create_formation_post) -> None:
    assert formation_api.create("test-formation") == Success([])


def test_given_create_formation_already_created_returns_409_http_error(
    formation_api, already_created_formation
) -> None:
    assert formation_api.create("test-formation") == Failure(
        HTTPError(409, "There is already a formation with that name")
    )


def test_given_get_all_api_call_returns_two_formations(formation_api, get_all_formations) -> None:
    assert formation_api.get_all() == Success(["formation-example", "test-formation"])


def test_given_get_all_api_call_returns_400_http_error(
    formation_api, fails_any_get  # noqa: F811
) -> None:
    assert formation_api.get_all() == Failure(HTTPError(400, "Some error"))


def test_given_get_all_api_call_includes_active_and_source_query_params(
    formation_api, create_formation_post_with_query_params
) -> None:
    assert formation_api.create("test-formation", active=True, source="any_source") == Success([])


def test_given_get_metadata_call_parses_the_response_correctly(
    formation_api, get_metadata
) -> None:
    assert formation_api.get_metadata("test-formation") == Success(
        FormationMetadata(url="https://example.url")
    )


def test_given_remove_formation_call_returns(formation_api, delete_formation) -> None:
    assert formation_api.delete("test-formation") == Success([])
