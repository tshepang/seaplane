from .api.lock_api import LockAPI
from .api.metadata_api import MetadataAPI
from .configuration import Configuration, config


class Seaplane:
    @property
    def config(self) -> Configuration:
        return config

    @property
    def metadata(self) -> MetadataAPI:
        return MetadataAPI(config)

    @property
    def locks(self) -> LockAPI:
        return LockAPI(config)


sea = Seaplane()
