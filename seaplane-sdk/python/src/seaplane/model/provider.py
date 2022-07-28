from enum import Enum
from typing import List, Optional


class Provider(Enum):
    aws = "AWS"
    azure = "Azure"
    digital_ocean = "DigitalOcean"
    equinix = "Equinix"
    gcp = "GCP"


def to_providers(providers: Optional[List[str]]) -> Optional[List[Provider]]:
    if not providers:
        return None

    return [Provider(provider) for provider in providers]
