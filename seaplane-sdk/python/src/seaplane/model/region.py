from enum import Enum
from typing import List, Optional


class Region(Enum):
    asia = "XA"
    republic_of_china = "XC"
    europe = "XE"
    africa = "XF"
    north_america = "XN"
    oceania = "XO"
    antartica = "XQ"
    south_america = "XS"
    uk = "XU"


def to_regions(regions: Optional[List[str]]) -> Optional[List[Region]]:
    if not regions:
        return None

    return [Region(region) for region in regions]
