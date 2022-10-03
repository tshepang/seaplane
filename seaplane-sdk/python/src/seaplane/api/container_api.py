from typing import Any, Text

import requests
from returns.result import Result

from ..configuration import Configuration, config
from ..model.errors import HTTPError
from .api_http import headers
from .api_request import provision_req


class ContainerAPI:
    """
    Class for handle Formation API calls.
    Link: https://developers.seaplane.io/reference/get_formations
    """

    def __init__(self, configuration: Configuration = config) -> None:
        self.url = f"{configuration.compute_endpoint}/formations"
        self.req = provision_req(configuration._token_api)

    def get_all(self, formation_name: Text) -> Result[Any, HTTPError]:
        url = f"{self.url}/{formation_name}/containers"

        return self.req(lambda access_token: requests.get(url, headers=headers(access_token)))

    def get(self, formation_name: Text, container_id: Text) -> Result[Any, HTTPError]:
        url = f"{self.url}/{formation_name}/containers/{container_id}"

        return self.req(lambda access_token: requests.get(url, headers=headers(access_token)))
