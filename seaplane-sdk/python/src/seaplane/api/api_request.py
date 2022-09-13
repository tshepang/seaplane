from typing import Any, Callable

import requests
from requests import Response
from returns.pipeline import is_successful
from returns.result import Failure, Result, Success

from ..model.errors import HTTPError
from .api_http import SDK_HTTP_ERROR_CODE
from .token_api import TokenAPI


def provision_req(
    token_api: TokenAPI,
) -> Callable[[Callable[[str], Response]], Result[Any, HTTPError]]:
    """
    Before every request, we make sure we use a valid access token.
    """

    def req(request: Callable[[str], Response]) -> Result[Any, HTTPError]:
        access_token = token_api.access_token()

        if is_successful(access_token):
            try:
                response = request(access_token.unwrap())

                if response.ok:
                    return Success(response.json())
                else:
                    return Failure(HTTPError(response.status_code, response.json()))
            except requests.exceptions.RequestException as err:
                return Failure(HTTPError(SDK_HTTP_ERROR_CODE, str(err)))
        else:
            return access_token

    return req
