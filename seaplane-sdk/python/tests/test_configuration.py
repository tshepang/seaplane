import os

from seaplane import Configuration, config
from seaplane.configuration import _SEAPLANE_ENV_VAR_API_KEY_NAME


def test_given_api_key_returns_none_when_not_initialized() -> None:
    assert config.seaplane_api_key is None


def test_given_api_key_returns_an_api_key_when_set_by_code() -> None:
    config.set_api_key("api_key")

    assert config.seaplane_api_key == "api_key"


def test_given_env_var_api_key_returns_it() -> None:
    os.environ[_SEAPLANE_ENV_VAR_API_KEY_NAME] = "env_var_api_key"
    new_config = Configuration()
    del os.environ[_SEAPLANE_ENV_VAR_API_KEY_NAME]

    assert new_config.seaplane_api_key == "env_var_api_key"


def test_given_env_var_api_key_override_it_when_set_by_code() -> None:
    os.environ[_SEAPLANE_ENV_VAR_API_KEY_NAME] = "env_var_api_key"
    new_config = Configuration()
    new_config.set_api_key("api_key")
    del os.environ[_SEAPLANE_ENV_VAR_API_KEY_NAME]

    assert config.seaplane_api_key == "api_key"


def test_given_configuration_set_endpoint_removing_last_slash() -> None:
    new_config = Configuration()

    new_config.set_endpoint("https://example.com/")

    assert new_config.endpoint == "https://example.com"


def test_given_configuration_set_endpoint_correctly() -> None:
    new_config = Configuration()

    new_config.set_endpoint("https://example.com")

    assert new_config.endpoint == "https://example.com"