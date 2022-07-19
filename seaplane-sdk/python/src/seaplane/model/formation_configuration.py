from typing import List, NamedTuple

from .flight import Flight, toFlights
from .provider import Provider
from .region import Region


class FormationConfiguration(NamedTuple):
    """
    Formation class with all formation attributes.
    """

    flights: List[Flight]
    affinity: List[str] | None = None
    connections: List[str] | None = None
    public_endpoints: object | None = None
    formation_endpoints: object | None = None
    flight_endpoints: object | None = None
    providers_allowed: List[Provider] | None = None
    providers_denied: List[Provider] | None = None
    regions_allowed: List[Region] | None = None
    regions_denied: List[Region] | None = None


def to_formation_config(dict: dict) -> FormationConfiguration:
    formation_config = dict.copy()
    del formation_config["flights"]
    return FormationConfiguration(**formation_config, flights=toFlights(dict["flights"]))
