from typing import List, NamedTuple, Optional

from .architecture import Architecture


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


def to_flights(flights: List[dict]) -> List[Flight]:
    return [Flight(**flight) for flight in flights]
