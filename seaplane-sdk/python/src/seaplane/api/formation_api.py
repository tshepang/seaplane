from typing import Any, Text

import requests
from returns.result import Result

from ..configuration import Configuration, config
from ..model import FormationMetadata
from .api_http import HTTPError, headers
from .api_request import provision_req
from .token_api import TokenAPI


class FormationAPI:
    """
    Class for handle Formation API calls.
    Link: https://developers.seaplane.io/reference/get_formations
    """

    def __init__(self, configuration: Configuration = config) -> None:
        self.url = f"{configuration.endpoint}/formations"
        self.req = provision_req(TokenAPI(configuration))

    def create(
        self, formation_name: str, active: bool = False, source: str | None = None
    ) -> Result[Any, HTTPError]:
        """
        Create a new formation

        Arguments:
            formation_name: a unique formation name.
            active: If this formation should be immediately deployed.
                    Note that this will only work if either the request body is
                    a configuration or the source parameter is set.
            source: The name of a formation this formation should be cloned from.
                    A copy of the source formation's configurations will be made under
                    this new formation.
                    If the active parameter is set, its active configuration will be copied over
                    and immediately deployed.
        """

        params = {"active": active}
        if source is not None:
            params["source"] = source

        return self.req(
            lambda access_token: requests.post(
                url=f"{self.url}/{formation_name}", params=params, headers=headers(access_token)
            )
        )

    def get_all(self) -> Result[[str], HTTPError]:
        return self.req(lambda access_token: requests.get(self.url, headers=headers(access_token)))

    def get_metadata(self, formation_name: Text) -> Result[FormationMetadata, HTTPError]:
        return self.req(
            lambda access_token: requests.get(
                url=f"{self.url}/{formation_name}", headers=headers(access_token)
            )
        ).map(lambda response: FormationMetadata(response["url"]))

    def delete(self, formation_name: Text) -> Result[Any, HTTPError]:
        return self.req(
            lambda access_token: requests.delete(
                url=f"{self.url}/{formation_name}", headers=headers(access_token)
            )
        )
