"""
End to End tests to Locks Coordination API.

This tests can't be ran more than once every 1 minute.
"""

import uuid

from returns.result import Success

from seaplane import sea
from seaplane.model import LockPage, Name

from . import E2E_API_KEY


def test_get_page_should_be_empty() -> None:
    sea.config.set_api_key(E2E_API_KEY)

    assert sea.locks.get_page() == Success(LockPage(locks=[], next_lock=None))


def test_acquire() -> None:
    sea.config.set_api_key(E2E_API_KEY)
    lock_name = Name(f"foo/bar/acquire/{uuid.uuid4()}".encode())
    acquire_result = sea.locks.acquire(lock_name, "test_acquire", 2)
    assert acquire_result.map(lambda x: x.sequencer) == Success(1)


def test_release_lock() -> None:
    sea.config.set_api_key(E2E_API_KEY)
    value = sea.locks.acquire(Name(b"foo/bar"), "test_release_lock", 60).unwrap()

    assert value.sequencer > 0
    assert sea.locks.release(Name(b"foo/bar"), value.id) == Success(True)
    assert sea.locks.get_page() == Success(LockPage(locks=[], next_lock=None))


def test_get_page_should_returns_one_element_when_acquire() -> None:
    sea.config.set_api_key(E2E_API_KEY)

    lock_name = Name(f"foo/bar/acquire/{uuid.uuid4()}".encode())
    sea.locks.acquire(lock_name, "test_acquire", 2)
    print(lock_name.name)
    assert sea.locks.get_page().map(lambda x: len(x.locks)) == Success(1)


def test_renew_lock() -> None:
    sea.config.set_api_key(E2E_API_KEY)

    lock_name = Name(f"foo/bar/acquire/{uuid.uuid4()}".encode())
    acquire = sea.locks.acquire(lock_name, "test_renew_lock", 10).unwrap()

    assert sea.locks.renew(lock_name, acquire.id, 15) == Success(True)
    assert sea.locks.get_all_pages().map(lambda x: [lock.info.ttl for lock in x]) == Success([15])


def test_get_all_pages_should_return_25_elements_using_pagination() -> None:
    sea.config.set_api_key(E2E_API_KEY)

    for i in range(0, 25):
        lock_name = Name(f"foo/bar/acquire/{uuid.uuid4()}".encode())
        sea.locks.acquire(lock_name, "test_get_all_pages_" + str(i), 60)

    locks = sea.locks.get_all_pages().unwrap()

    assert len(locks) == 25
