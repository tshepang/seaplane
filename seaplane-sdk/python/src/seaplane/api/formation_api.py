from typing import Any, Dict, List, Optional, Text

import requests
from returns.result import Result

from ..configuration import Configuration, config
from ..model.compute.formation_metadata import FormationMetadata
from ..model.errors import HTTPError
from .api_http import headers
from .api_request import provision_req


class FormationAPI:
    """
    Class for handle Formation API calls.
    Link: https://developers.seaplane.io/reference/get_formations
    """

    def __init__(self, configuration: Configuration = config) -> None:
        self.url = f"{configuration.compute_endpoint}/formations"
        self.req = provision_req(configuration._token_api)

    def create(
        self,
        formation_name: str,
        active: bool = False,
        source: Optional[str] = None,
        token: Optional[str] = None,
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

        params: Dict[str, Any] = {"active": active}
        if source is not None:
            params["source"] = source

        return self.req(
            lambda access_token: requests.post(
                url=f"{self.url}/{formation_name}", params=params, headers=headers(access_token)
            ),
            token,
        )

    def get_all(self, token: Optional[str] = None) -> Result[List[int], HTTPError]:
        return self.req(
            lambda access_token: requests.get(self.url, headers=headers(access_token)), token
        )

    def get_metadata(
        self, formation_name: Text, token: Optional[str] = None
    ) -> Result[FormationMetadata, HTTPError]:
        return self.req(
            lambda access_token: requests.get(
                url=f"{self.url}/{formation_name}", headers=headers(access_token)
            ),
            token,
        ).map(lambda response: FormationMetadata(response["url"]))

    def delete(self, formation_name: Text, token: Optional[str] = None) -> Result[Any, HTTPError]:
        return self.req(
            lambda access_token: requests.delete(
                url=f"{self.url}/{formation_name}", headers=headers(access_token)
            ),
            token,
        )
