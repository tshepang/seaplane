from typing import Text

import requests
from returns.result import Failure, Result, Success

from ..model.errors import HTTPError
from ..util import unwrap
from .api_http import SDK_HTTP_ERROR_CODE, headers


class TokenAPI:
    """
    Manage access token from seaplane api.
    There isn't documentation for token management, as it may change soon.

    The access token will expire in 60 seconds.
    If you have a couple api calls to make in succession you can reuse the token.
    Otherwise get a new one each time.

    `curl -H "Authorization: Bearer $api_key" -X POST https://identity.cplane.cloud/token`
    """

    def __init__(self, configuration) -> None:  # type: ignore
        self.url = f"{configuration.identify_endpoint}/token"
        self.api_key = configuration.seaplane_api_key
        self._current_access_token = None

    def set_identify_url(self, url: Text) -> None:
        self.url = url

    def get_token(self) -> Text:
        return unwrap(self._request_access_token())

    def _request_access_token(self) -> Result[Text, HTTPError]:
        try:
            response = requests.post(self.url, json={}, headers=headers(self.api_key))

            if response.ok:
                access_token = response.json()["token"]
                self._current_access_token = access_token
                return Success(access_token)
            else:
                self._current_access_token = None
                return Failure(HTTPError(response.status_code, response.json()))

        except requests.exceptions.RequestException as err:
            self._current_access_token = None
            return Failure(HTTPError(SDK_HTTP_ERROR_CODE, "[TokenAPI]: " + str(err)))
