from enum import Enum
from typing import List, Optional


class Architecture(Enum):
    amd64 = "amd64"
    arm64 = "arm64"


def to_architectures(architectures: Optional[List[str]]) -> Optional[List[Architecture]]:
    if architectures is None:
        return None
    return [Architecture(architecture) for architecture in architectures]
