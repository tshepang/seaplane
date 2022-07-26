from typing import List, NamedTuple

from .architecture import Architecture


class Flight(NamedTuple):
    """
    Flight class.
    """

    name: str
    image: str
    minimum: int | None = None
    maximum: int | None = None
    architecture: List[Architecture] | None = None
    api_permission: bool | None = None


def to_flights(flights: List[dict]) -> List[Flight]:
    return [Flight(**flight) for flight in flights]
