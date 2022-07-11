import os
from sre_constants import SRE_FLAG_TEMPLATE
from typing import Text

SEAPLANE_ENV_VAR_API_KEY_NAME = "SEAPLANE_API_KEY"


class Configuration:
    """
    Access all configuration SDK
    """

    def __init__(self) -> None:
        self.seaplane_api_key = None
        env_api_key = os.getenv(SEAPLANE_ENV_VAR_API_KEY_NAME)

        if env_api_key is not None:
            self.seaplane_api_key = env_api_key

    def set_api_key(self, api_key: Text) -> None:
        self.seaplane_api_key = api_key


config = Configuration()
