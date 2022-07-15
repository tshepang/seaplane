from enum import Enum


class Provider(Enum):
    aws = "AWS"
    azure = "Azure"
    digital_ocean = "DigitalOcean"
    equinix = "Equinix"
    gcp = "GCP"
