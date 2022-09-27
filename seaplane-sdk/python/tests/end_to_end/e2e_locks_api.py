"""
End to End tests to Locks Coordination API.

This tests can't be ran more than once every 1 minute.
"""

import uuid

from seaplane import sea
from seaplane.model import LockPage, Name

from . import E2E_API_KEY


def test_get_page_should_be_empty() -> None:
    sea.config.set_api_key(E2E_API_KEY)

    assert sea.locks.get_page() == LockPage(locks=[], next_lock=None)


def test_acquire() -> None:
    sea.config.set_api_key(E2E_API_KEY)
    lock_name = Name(f"foo/bar/acquire/{uuid.uuid4()}".encode())
    acquire_result = sea.locks.acquire(lock_name, "test_acquire", 2)
    assert acquire_result.sequencer == 1


def test_release_lock() -> None:
    sea.config.set_api_key(E2E_API_KEY)
    value = sea.locks.acquire(Name(b"foo/bar"), "test_release_lock", 60)

    assert value.sequencer > 0
    assert sea.locks.release(Name(b"foo/bar"), value.id)
    assert sea.locks.get_page() == LockPage(locks=[], next_lock=None)


def test_get_page_should_returns_one_element_when_acquire() -> None:
    sea.config.set_api_key(E2E_API_KEY)

    lock_name = Name(f"foo/bar/acquire/{uuid.uuid4()}".encode())
    sea.locks.acquire(lock_name, "test_acquire", 2)

    assert len(sea.locks.get_page().locks) == 1


def test_renew_lock() -> None:
    sea.config.set_api_key(E2E_API_KEY)

    lock_name = Name(f"foo/bar/acquire/{uuid.uuid4()}".encode())
    acquire = sea.locks.acquire(lock_name, "test_renew_lock", 10)

    assert sea.locks.renew(lock_name, acquire.id, 15)
    assert sea.locks.get_all_pages()[0].info.ttl == 15


def test_get_all_pages_should_return_25_elements_using_pagination() -> None:
    sea.config.set_api_key(E2E_API_KEY)

    for i in range(0, 25):
        lock_name = Name(f"foo/bar/acquire/{uuid.uuid4()}".encode())
        sea.locks.acquire(lock_name, "test_get_all_pages_" + str(i), 60)

    assert len(sea.locks.get_all_pages()) == 25
