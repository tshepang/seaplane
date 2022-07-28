from typing import Any, Dict, List, NamedTuple, Optional

from .flight import Flight, to_flights


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
    providers_allowed: Optional[List[str]] = None
    providers_denied: Optional[List[str]] = None
    regions_allowed: Optional[List[str]] = None
    regions_denied: Optional[List[str]] = None


def to_formation_config(fconfig: Dict[str, Any]) -> FormationConfiguration:
    formation_config = fconfig.copy()
    del formation_config["flights"]
    return FormationConfiguration(**formation_config, flights=to_flights(fconfig["flights"]))
