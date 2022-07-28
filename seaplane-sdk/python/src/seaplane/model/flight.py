from typing import Any, Dict, List, NamedTuple, Optional


class Flight(NamedTuple):
    """
    Flight class.
    """

    name: str
    image: str
    minimum: Optional[int] = None
    maximum: Optional[int] = None
    architecture: Optional[List[str]] = None
    api_permission: Optional[bool] = None


def to_flights(flights: List[Dict[str, Any]]) -> List[Flight]:
    return [Flight(**flight) for flight in flights]
