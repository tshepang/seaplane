import os
from typing import Optional


def get_absolute_path(relative_path: str) -> str:
    return os.path.join(os.path.dirname(__file__), relative_path)


def get_file_bytes(
    absolute_path: Optional[str] = None, relative_path: Optional[str] = None
) -> bytes:
    path = ""

    if absolute_path is not None:
        path = absolute_path
    elif relative_path is not None:
        path = get_absolute_path(relative_path)

    content = b""
    with open(path, "rb") as image:
        content = image.read()

    return content
