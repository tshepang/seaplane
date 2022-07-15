from enum import Enum


class ContainerStatus(Enum):
    started = "started"
    running = "running"
    stopped = "stopped"
