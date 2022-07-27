from typing import Text

_SEAPLANE_API_ENDPOINT = "https://compute.cplane.cloud/v1"


class Configuration:
    """
    Access all configuration SDK.
    """

    def __init__(self) -> None:
        self.seaplane_api_key: Text | None = None
        self.endpoint = _SEAPLANE_API_ENDPOINT

    def set_api_key(self, api_key: Text) -> None:
        self.seaplane_api_key = api_key

    def set_endpoint(self, endpoint: str) -> None:
        if endpoint[-1] == "/":
            self.endpoint = endpoint.rstrip(endpoint[-1])
        else:
            self.endpoint = endpoint


config = Configuration()
