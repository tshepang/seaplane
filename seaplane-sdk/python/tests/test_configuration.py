from seaplane.configuration import Configuration, config


def test_given_api_key_returns_none_when_not_initialized() -> None:
    assert config.seaplane_api_key == ""


def test_given_api_key_returns_an_api_key_when_set_by_code() -> None:
    config.set_api_key("api_key")

    assert config.seaplane_api_key == "api_key"


def test_given_configuration_set_endpoint_removing_last_slash() -> None:
    new_config = Configuration()

    new_config.set_endpoint("https://example.com/")

    assert new_config.endpoint == "https://example.com"


def test_given_configuration_set_endpoint_correctly() -> None:
    new_config = Configuration()

    new_config.set_endpoint("https://example.com")

    assert new_config.endpoint == "https://example.com"
