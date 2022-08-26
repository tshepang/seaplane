from .api.metadata_api import MetadataAPI
from .configuration import Configuration, config


class Seaplane:
    @property
    def config(self) -> Configuration:
        return config

    @property
    def metadata(self) -> MetadataAPI:
        return MetadataAPI(config)


sea = Seaplane()
