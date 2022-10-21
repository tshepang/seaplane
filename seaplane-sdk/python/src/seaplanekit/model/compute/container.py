from typing import NamedTuple, Optional

from .provider import Provider
from .region import Region
from .status import ContainerStatus


class Container(NamedTuple):
    """
    Container class.
    """

    container_id: str
    configuration_id: str
    flight_name: str
    status: ContainerStatus
    exit_status: Optional[int] = None
    start_time: Optional[str] = None
    stop_time: Optional[str] = None
    host_latitude: Optional[float] = None
    host_longitude: Optional[float] = None
    host_iata: Optional[str] = None
    host_country: Optional[str] = None
    host_region: Optional[Region] = None
    host_provider: Optional[Provider] = None
    public_ingress_usage: Optional[int] = None
    public_egress_usage: Optional[int] = None
    private_ingress_usage: Optional[int] = None
    private_egress_usage: Optional[int] = None
    disk_usage: Optional[int] = None
    ram_usage: Optional[int] = None
    cpu_usage: Optional[int] = None
