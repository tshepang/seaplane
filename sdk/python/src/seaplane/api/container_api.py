from typing import Any, Text

import requests
from returns.result import Failure, Result, Success

from ..configuration import Configuration, config
from ..model import Container
from .api_http import HTTPError, headers, to_json
from .api_request import provisionReq
from .token_api import TokenAPI


class ContainerAPI:
    """
    Class for handle Formation API calls.
    Link: https://developers.seaplane.io/reference/get_formations
    """

    def __init__(self, configuration: Configuration = config) -> None:
        self.url = f"{configuration.endpoint}/formations"
        self.req = provisionReq(TokenAPI(configuration))

    def get_all(self, formation_name: Text) -> Result[Any, HTTPError]:
        url = f"{self.url}/{formation_name}/containers"

        return self.req(lambda access_token: requests.get(url, headers=headers(access_token)))

    def get(self, formation_name: Text, container_id: Text) -> Result[Any, HTTPError]:
        url = f"{self.url}/{formation_name}/containers/{container_id}"

        return self.req(lambda access_token: requests.get(url, headers=headers(access_token)))
