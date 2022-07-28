from typing import Any, Dict, List, NamedTuple, Optional, Tuple

from .architecture import Architecture, to_architectures


class Flight(NamedTuple):
    """
    Flight class.
    """

    name: str
    image: str
    minimum: Optional[int] = None
    maximum: Optional[int] = None
    architecture: Optional[List[Architecture]] = None
    api_permission: Optional[bool] = None


def to_flights(flights: List[Dict[str, Any]]) -> List[Flight]:
    return [
        Flight(**flight[0], architecture=to_architectures(flight[1]))
        for flight in _extract_arch(flights)
    ]


def _extract_arch(flights: List[Dict[str, Any]]) -> List[Tuple[Dict[str, Any], List[str]]]:
    def remove_arch(flight: Dict[str, Any]) -> Tuple[Dict[str, Any], List[str]]:
        arch = flight["architecture"]
        flight.pop("architecture", None)
        return (flight, arch)

    return [remove_arch(flight) for flight in flights]
