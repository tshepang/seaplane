from typing import NamedTuple

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
    exit_status: int | None = None
    start_time: str | None = None
    stop_time: str | None = None
    host_latitude: float | None = None
    host_longitude: float | None = None
    host_iata: str | None = None
    host_country: str | None = None
    host_region: Region | None = None
    host_provider: Provider | None = None
    public_ingress_usage: int | None = None
    public_egress_usage: int | None = None
    private_ingress_usage: int | None = None
    private_egress_usage: int | None = None
    disk_usage: int | None = None
    ram_usage: int | None = None
    cpu_usage: int | None = None
