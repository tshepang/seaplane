from typing import NamedTuple

import simplejson as json

SDK_HTTP_ERROR_CODE = 0


class HTTPError(NamedTuple):
    status: int
    message: str = ""


def headers(api_key: str) -> dict[str, str]:
    return {
        "Accept": "application/json",
        "Content-Type": "application/json",
        "Authorization": f"Bearer {api_key}",
    }


def to_json(any: NamedTuple) -> str:
    return json.loads(json.dumps(any), object_hook=_remove_nulls)


def _remove_nulls(d):
    return {k: v for k, v in d.items() if v is not None}
