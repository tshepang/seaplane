from typing import Any, Callable, Optional

import requests
from requests import Response
from returns.pipeline import is_successful
from returns.result import Failure, Result, Success

from ..model.errors import HTTPError
from .api_http import SDK_HTTP_ERROR_CODE
from .token_api import TokenAPI


def provision_req(
    token_api: TokenAPI,
) -> Callable[[Callable[[str], Response], Optional[str]], Result[Any, HTTPError]]:
    """
    Before every request, we make sure we use a valid access token.
    """

    def handle_request(request: Callable[[str], Response], token: str) -> Result[Any, HTTPError]:
        try:
            response = request(token)

            if response.ok:
                return Success(response.json())
            else:
                return Failure(HTTPError(response.status_code, response.json()))
        except requests.exceptions.RequestException as err:
            return Failure(HTTPError(SDK_HTTP_ERROR_CODE, str(err)))

    def req(
        request: Callable[[str], Response], token: Optional[str] = None
    ) -> Result[Any, HTTPError]:
        if token is not None:
            return handle_request(request, token)
        else:
            access_token = token_api._request_access_token()

            if is_successful(access_token):
                return handle_request(request, access_token.unwrap())
            else:
                return access_token

    return req
