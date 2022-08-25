from typing import Text

import requests
from returns.result import Failure, Result, Success

from ..configuration import Configuration, config
from .api_http import SDK_HTTP_ERROR_CODE, HTTPError, headers


class TokenAPI:
    """
    Manage access token from seaplane api.
    There isn't documentation for token management, as it may change soon.

    In order to get the Access Token,
    we have to do this call everytime we want to perform an API call.
    The access token will expire in 60 seconds.
    If you have a couple api calls to make in succession you can reuse the token.
    Otherwise get a new one each time.

    `curl -H "Authorization: Bearer $api_key" -X POST https://identity.cplane.cloud/token`
    """

    def __init__(self, configuration: Configuration = config) -> None:
        self.url = f"{configuration.identify_endpoint}/token"
        self.api_key = configuration.seaplane_api_key

    def set_identify_url(self, url: Text) -> None:
        self.url = url

    def access_token(self) -> Result[Text, HTTPError]:
        try:
            response = requests.post(self.url, json={}, headers=headers(self.api_key))

            if response.ok:
                return Success(response.json()["token"])
            else:
                return Failure(HTTPError(response.status_code, response.json()))

        except requests.exceptions.RequestException as err:
            return Failure(HTTPError(SDK_HTTP_ERROR_CODE, "[TokenAPI]: " + str(err)))


token_api = TokenAPI()
