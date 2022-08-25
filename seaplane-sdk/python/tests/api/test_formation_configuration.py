from typing import Any, Generator

import pytest
import requests_mock
from returns.result import Failure, Success

from seaplane.api.api_http import HTTPError
from seaplane.api.formation_configuration_api import FormationConfigurationAPI
from seaplane.configuration import Configuration
from seaplane.model.compute.architecture import Architecture
from seaplane.model.compute.flight import Flight
from seaplane.model.compute.formation_configuration import FormationConfiguration
from seaplane.model.compute.provider import Provider
from seaplane.model.compute.region import Region

from ..conftest import add_token_request


@pytest.fixture
def create_formation_config_minimum_setup() -> Generator[None, None, None]:
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        def match_authorization(request: Any) -> Any:
            return (
                request.headers["Authorization"] == "Bearer This is a token"
                and request.query == "active=false"
                and request.json()
                == {
                    "flights": [{"name": "flight-name", "image": "flight-image"}],
                }
            )

        requests_mocker.post(
            "https://compute.cplane.cloud/v1/formations/test-formation/configurations",
            additional_matcher=match_authorization,
            status_code=201,
            json="65d72648-0e67-402c-9b8e-f56e2d6e2331",
        )

        yield


@pytest.fixture
def get_all_configurations_ids() -> Generator[None, None, None]:
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        def match_authorization(request: Any) -> Any:
            return request.headers["Authorization"] == "Bearer This is a token"

        requests_mocker.get(
            "https://compute.cplane.cloud/v1/formations/test-formation/configurations",
            additional_matcher=match_authorization,
            status_code=200,
            json=["65d72648-0e67-402c-9b8e-f56e2d6e2331", "22d72648-0e67-402c-9b8e-f56e2d6e2222"],
        )

        yield


@pytest.fixture
def get_configuration_by_id() -> Generator[None, None, None]:
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        def match_authorization(request: Any) -> Any:
            return request.headers["Authorization"] == "Bearer This is a token"

        requests_mocker.get(
            "https://compute.cplane.cloud/v1/formations/test-formation/configurations/22d72648-0e67-402c-9b8e-f56e2d6e2222",  # noqa: E501
            additional_matcher=match_authorization,
            status_code=200,
            json={
                "flights": [
                    {
                        "minimum": 1,
                        "architecture": ["amd64"],
                        "api_permission": False,
                        "name": "some_flight",
                        "image": "https://registry.io",
                        "maximum": 2,
                    }
                ],
                "providers_denied": ["DigitalOcean"],
                "regions_allowed": ["XE"],
                "regions_denied": ["XF"],
            },
        )

        yield


@pytest.fixture
def fails_create_formation_config() -> Generator[None, None, None]:
    with requests_mock.Mocker() as requests_mocker:
        add_token_request(requests_mocker)

        requests_mocker.post(
            "https://compute.cplane.cloud/v1/formations/test-formation/configurations",
            status_code=400,
            json="Some error",
        )

        yield


@pytest.fixture
def formation_configuration() -> Generator[FormationConfigurationAPI, None, None]:
    config = Configuration()
    config.set_api_key("api_key")
    formation_configuration = FormationConfigurationAPI(config)

    yield formation_configuration


def any_formation_config() -> FormationConfiguration:
    flight = Flight(name="flight-name", image="flight-image")
    return FormationConfiguration(flights=[flight])


def test_given_create_configuration_call_creates_it_with_minimum_setup(  # type: ignore
    formation_configuration, create_formation_config_minimum_setup
) -> None:
    assert formation_configuration.create("test-formation", any_formation_config()) == Success(
        "65d72648-0e67-402c-9b8e-f56e2d6e2331"
    )


def test_given_create_formation_configuration_returns_400_error(  # type: ignore
    formation_configuration, fails_create_formation_config
) -> None:
    assert formation_configuration.create("test-formation", any_formation_config()) == Failure(
        HTTPError(400, "Some error")
    )


def test_given_get_all_configurations_returns_them_correctly(  # type: ignore
    formation_configuration, get_all_configurations_ids
) -> None:
    assert formation_configuration.get_all("test-formation") == Success(
        ["65d72648-0e67-402c-9b8e-f56e2d6e2331", "22d72648-0e67-402c-9b8e-f56e2d6e2222"]
    )


def test_given_get_configuration_by_id_returns_it_correctly(  # type: ignore
    formation_configuration, get_configuration_by_id
) -> None:
    assert formation_configuration.get(
        "test-formation", "22d72648-0e67-402c-9b8e-f56e2d6e2222"
    ) == Success(
        FormationConfiguration(
            flights=[
                Flight(
                    name="some_flight",
                    image="https://registry.io",
                    minimum=1,
                    maximum=2,
                    architecture=[Architecture.amd64],
                    api_permission=False,
                )
            ],
            providers_denied=[Provider.digital_ocean],
            regions_allowed=[Region.europe],
            regions_denied=[Region.africa],
        )
    )
