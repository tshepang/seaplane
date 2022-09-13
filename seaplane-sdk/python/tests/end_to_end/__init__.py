"""
End to End tests.

You have to set E2E_API_KEY env var to run e2e tests.
"""

import os

E2E_API_KEY = os.getenv("E2E_API_KEY") or ""
