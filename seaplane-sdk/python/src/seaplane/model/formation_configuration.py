from typing import List, NamedTuple, Optional

from .flight import Flight, to_flights
from .provider import Provider
from .region import Region


class FormationConfiguration(NamedTuple):
    """
    Formation class with all formation attributes.
    """

    flights: List[Flight]
    affinity: Optional[List[str]] = None
    connections: Optional[List[str]] = None
    public_endpoints: Optional[object] = None
    formation_endpoints: Optional[object] = None
    flight_endpoints: Optional[object] = None
    providers_allowed: Optional[List[Provider]] = None
    providers_denied: Optional[List[Provider]] = None
    regions_allowed: Optional[List[Region]] = None
    regions_denied: Optional[List[Region]] = None


def to_formation_config(dict: dict) -> FormationConfiguration:
    formation_config = dict.copy()
    del formation_config["flights"]
    return FormationConfiguration(**formation_config, flights=to_flights(dict["flights"]))
