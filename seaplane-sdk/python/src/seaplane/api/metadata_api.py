from typing import Any, Dict, Optional

import requests
from returns.result import Result

from ..configuration import Configuration, config
from ..model.metadata.key_value import (
    Key,
    KeyValue,
    KeyValueRange,
    to_key_value,
    to_key_value_range,
)
from ..util.base64url import base64url_encode_from_bytes
from .api_http import HTTPError, headers
from .api_request import provision_req
from .token_api import TokenAPI


class MetadataAPI:
    """
    Class for handle Metadata API calls.
    """

    def __init__(self, configuration: Configuration = config) -> None:
        self.url = f"{configuration.coordination_endpoint}/config"
        self.req = provision_req(TokenAPI(configuration))

    def set(self, key_value: KeyValue) -> Result[Any, HTTPError]:
        """Adds a value to the store at the given key.

        Parameters
        ----------
        key_value : KeyValue
            key-value pair to be set, for example, key=foo/bar, value=hello

        Returns
        -------
        Result[boolean, HTTPError]
            Returns true if sucess, you will get an HTTPError.
        """
        url = f"{self.url}/base64:{base64url_encode_from_bytes(key_value.key)}"

        return self.req(
            lambda access_token: requests.put(
                url,
                data=base64url_encode_from_bytes(key_value.value),
                headers=headers(access_token),
            )
        ).map(lambda success: success == "Ok")

    def get_page(self, next_key: Optional[Key] = None) -> Result[KeyValueRange, HTTPError]:
        """Returns a single page of key value pairs, beginning with the `next_key` key.

        Parameters
        ----------
        next_key : Optional[Key]
            If next_key is not null, you can repeat this query with the next_key parameter
            set to the next_key value from the previous query to see the next page of results.

        Returns
        -------
        Result[KeyValueRange, HTTPError]
            Returns KeyValueRange if sucess, you will get an HTTPError otherwise.
        """

        params: Dict[str, Any] = {}
        if next_key is not None:
            params["from"] = "base64:" + base64url_encode_from_bytes(next_key.key)

        return self.req(
            lambda access_token: requests.get(
                self.url, params=params, headers=headers(access_token)
            )
        ).map(lambda key_value_range: to_key_value_range(key_value_range))

    def get(self, key: Key) -> Result[KeyValue, HTTPError]:
        """Returns the key value pair associated with the set key.

        Parameters
        ----------
        key : Key
            The key from a key-value previously set, for example, key=foo/bar

        Returns
        -------
        Result[KeyValue, HTTPError]
            Returns KeyValue if sucess, you will get an HTTPError otherwise.
        """
        url = f"{self.url}/base64:{base64url_encode_from_bytes(key.key)}"

        return self.req(lambda access_token: requests.get(url, headers=headers(access_token))).map(
            lambda key_value: to_key_value(key_value)
        )

    def delete(self, key: Key) -> Result[Any, HTTPError]:
        """Deletes the key value pair at from a given key.

        Parameters
        ----------
        directory : Key
            The key from a key-value previously set, for example, key=foo/bar

        Returns
        -------
        Result[boolean, HTTPError]
            Returns true if sucess, you will get an HTTPError otherwise.
        """
        url = f"{self.url}/base64:{base64url_encode_from_bytes(key.key)}"

        return self.req(
            lambda access_token: requests.delete(url, headers=headers(access_token))
        ).map(lambda success: success == "Ok")
