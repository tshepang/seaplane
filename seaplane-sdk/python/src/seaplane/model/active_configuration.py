from typing import NamedTuple, Optional


class ActiveConfiguration(NamedTuple):
    """
    Active configuration class with the actual active configuration from a formation.
    """

    configuration_id: str
    traffic_weight: Optional[int] = None
