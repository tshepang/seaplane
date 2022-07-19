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


def toFlights(flights: List[dict]) -> List[Flight]:
    return list(map(lambda flight: Flight(**flight), flights))
