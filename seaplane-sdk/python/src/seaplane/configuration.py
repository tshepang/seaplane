from typing import Text

_SEAPLANE_COMPUTE_API_ENDPOINT = "https://compute.cplane.cloud/v1"
_SEAPLANE_COORDINATION_API_ENDPOINT = "https://metadata.cplane.cloud/v1"
_SEAPLANE_IDENTIFY_API_ENDPOINT = "https://identity.cplane.cloud"


class Configuration:
    """
    Access all configuration SDK.
    """

    def __init__(self) -> None:
        self.seaplane_api_key: Text = ""
        self.identify_endpoint = _SEAPLANE_IDENTIFY_API_ENDPOINT
        self.compute_endpoint = _SEAPLANE_COMPUTE_API_ENDPOINT
        self.coordination_endpoint = _SEAPLANE_COORDINATION_API_ENDPOINT

    def set_api_key(self, api_key: Text) -> None:
        self.seaplane_api_key = api_key

    def set_compute_endpoint(self, endpoint: str) -> None:
        if endpoint[-1] == "/":
            self.compute_endpoint = endpoint.rstrip(endpoint[-1])
        else:
            self.compute_endpoint = endpoint

    def set_coordination_endpoint(self, endpoint: str) -> None:
        if endpoint[-1] == "/":
            self.coordination_endpoint = endpoint.rstrip(endpoint[-1])
        else:
            self.coordination_endpoint = endpoint

    def set_identify_endpoint(self, endpoint: str) -> None:
        if endpoint[-1] == "/":
            self.identify_endpoint = endpoint.rstrip(endpoint[-1])
        else:
            self.identify_endpoint = endpoint


config = Configuration()
