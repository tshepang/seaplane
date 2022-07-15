from typing import Any, Callable, TypeVar

import requests
from requests import Response
from returns.pipeline import is_successful
from returns.result import Failure, Result, Success

from .api_http import SDK_HTTP_ERROR_CODE, HTTPError
from .token_api import TokenAPI

T = TypeVar("T")


def provisionReq(
    token_api: TokenAPI,
) -> Callable[[Callable[[str], Response]], Result[T, HTTPError]]:
    def req(request: Callable[[str], Response]) -> Result[T, HTTPError]:
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
