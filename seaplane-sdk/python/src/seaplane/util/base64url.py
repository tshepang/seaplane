from base64 import urlsafe_b64decode, urlsafe_b64encode
from typing import Optional


def base64url_encode_from_bytes(data: Optional[bytes]) -> Optional[str]:
    if data is None:
        return None

    return urlsafe_b64encode(data).rstrip(b"=").decode()


def base64url_decode_to_bytes(data: Optional[str]) -> Optional[bytes]:
    if data is None:
        return None

    padding = b"=" * (4 - (len(data) % 4))
    return urlsafe_b64decode(data.encode() + padding)


def base64url_encode(data: Optional[str], encoding: str = "utf-8") -> Optional[str]:
    if data is None:
        return None

    return urlsafe_b64encode(data.encode(encoding)).rstrip(b"=").decode()


def base64url_decode(data: Optional[str], enconding: str = "utf-8") -> Optional[str]:
    if data is None:
        return None

    padding = b"=" * (4 - (len(data) % 4))
    return urlsafe_b64decode(data.encode(enconding) + padding).decode()
