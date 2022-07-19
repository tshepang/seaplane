from typing import List, NamedTuple


class ActiveConfiguration(NamedTuple):
    """
    Active configuration class with the actual active configuration from a formation.
    """

    configuration_id: str
    traffic_weight: int | None = None
